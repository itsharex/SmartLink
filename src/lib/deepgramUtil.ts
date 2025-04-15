// deepgramUtils.ts
import { createClient, LiveTranscriptionEvents } from '@deepgram/sdk';

export class DeepgramTranscriber {
  private deepgram: ReturnType<typeof createClient>;
  private connection: any;
  private mediaRecorder: MediaRecorder | null = null;
  private onTranscript: (text: string) => void;

  constructor(apiKey: string, onTranscript: (text: string) => void) {
    this.deepgram = createClient(apiKey);
    this.onTranscript = onTranscript;
  }

  async startRecording() {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      this.mediaRecorder = new MediaRecorder(stream, {
        mimeType: 'audio/webm',
      });

      this.connection = this.deepgram.listen.live({
        model: 'nova-2',
        language: 'en-US',
        smart_format: true,
        interim_results: true,
        endpointing: 300, // 300ms of silence to mark end of speech
      });

      this.connection.on(LiveTranscriptionEvents.Open, () => {
        this.setupEventListeners();
      });

      this.mediaRecorder.ondataavailable = (event) => {
        if (this.connection && event.data.size > 0) {
          this.connection.send(event.data);
        }
      };

      this.mediaRecorder.onstop = () => {
        this.cleanup();
      };

      this.mediaRecorder.start(100); // Send audio data every 100ms
    } catch (error) {
      console.error('Error starting recording:', error);
    }
  }

  private setupEventListeners() {
    this.connection.on(LiveTranscriptionEvents.Transcript, (data: any) => {
      const transcript = data.channel?.alternatives[0]?.transcript;
      if (transcript && data.is_final) {
        this.onTranscript(transcript);
      }
    });

    this.connection.on(LiveTranscriptionEvents.Close, () => {
      console.log('Deepgram connection closed');
    });

    this.connection.on(LiveTranscriptionEvents.Error, (error: any) => {
      console.error('Deepgram error:', error);
    });
  }

  stopRecording() {
    if (this.mediaRecorder && this.mediaRecorder.state !== 'inactive') {
      this.mediaRecorder.stop();
    }
    if (this.connection) {
      this.connection.finish();
    }
  }

  private cleanup() {
    if (this.mediaRecorder) {
      this.mediaRecorder.stream.getTracks().forEach(track => track.stop());
      this.mediaRecorder = null;
    }
  }
}

export default DeepgramTranscriber;