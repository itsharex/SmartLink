import React, { useState, useRef, useEffect } from 'react';
import Button from '../ui/Button';
import { Mic, Square, Trash2, Send, Pause, Play } from 'lucide-react';

type VoiceMessageRecorderProps = {
  onSend: (audioBlob: Blob, duration: number) => void;
  onCancel: () => void;
};

const VoiceMessageRecorder: React.FC<VoiceMessageRecorderProps> = ({
  onSend,
  onCancel
}) => {
  const [isRecording, setIsRecording] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const [duration, setDuration] = useState(0);
  const [audioBlob, setAudioBlob] = useState<Blob | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [visualData, setVisualData] = useState<number[]>(Array(30).fill(0));
  
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioChunksRef = useRef<BlobPart[]>([]);
  const timerRef = useRef<NodeJS.Timeout | null>(null);
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const analyserRef = useRef<AnalyserNode | null>(null);
  const dataArrayRef = useRef<Uint8Array | null>(null);
  
  // 初始化音频上下文和分析器
  useEffect(() => {
    let audioContext: AudioContext | null = null;
    
    if (isRecording && !isPaused) {
      audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
      analyserRef.current = audioContext.createAnalyser();
      analyserRef.current.fftSize = 64;
      
      const bufferLength = analyserRef.current.frequencyBinCount;
      dataArrayRef.current = new Uint8Array(bufferLength);
      
      const updateVisualizer = () => {
        if (!analyserRef.current || !dataArrayRef.current) return;
        
        analyserRef.current.getByteFrequencyData(dataArrayRef.current);
        
        // 更新可视化数据
        const newData = Array.from(dataArrayRef.current)
          .slice(0, 30)
          .map(val => val / 255); // 归一化
        
        setVisualData(newData);
        animationFrameRef.current = requestAnimationFrame(updateVisualizer);
      };
      
      // 开始可视化
      updateVisualizer();
    }
    
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
      if (audioContext) {
        audioContext.close();
      }
    };
  }, [isRecording, isPaused]);
  
  // 开始录音
  const startRecording = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      mediaRecorderRef.current = new MediaRecorder(stream);
      
      mediaRecorderRef.current.ondataavailable = (event) => {
        audioChunksRef.current.push(event.data);
      };
      
      mediaRecorderRef.current.onstop = () => {
        const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/webm' });
        setAudioBlob(audioBlob);
        
        // 连接到音频上下文进行可视化
        if (analyserRef.current) {
          const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
          const source = audioContext.createMediaStreamSource(stream);
          source.connect(analyserRef.current);
        }
      };
      
      audioChunksRef.current = [];
      mediaRecorderRef.current.start();
      
      // 开始计时
      setIsRecording(true);
      startTimer();
      
    } catch (err) {
      console.error('无法访问麦克风', err);
    }
  };
  
  // 暂停录音
  const pauseRecording = () => {
    if (mediaRecorderRef.current && isRecording) {
      setIsPaused(true);
      mediaRecorderRef.current.pause();
      stopTimer();
    }
  };
  
  // 恢复录音
  const resumeRecording = () => {
    if (mediaRecorderRef.current && isRecording && isPaused) {
      setIsPaused(false);
      mediaRecorderRef.current.resume();
      startTimer();
    }
  };
  
  // 停止录音
  const stopRecording = () => {
    if (mediaRecorderRef.current && (isRecording || isPaused)) {
      mediaRecorderRef.current.stop();
      
      // 停止所有轨道
      if (mediaRecorderRef.current.stream) {
        mediaRecorderRef.current.stream.getTracks().forEach(track => track.stop());
      }
      
      setIsRecording(false);
      setIsPaused(false);
      stopTimer();
    }
  };
  
  // 计时器
  const startTimer = () => {
    if (timerRef.current) clearInterval(timerRef.current);
    
    timerRef.current = setInterval(() => {
      setDuration(prev => prev + 1);
    }, 1000);
  };
  
  const stopTimer = () => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }
  };
  
  // 播放录音
  const playRecording = () => {
    if (audioBlob && audioRef.current) {
      audioRef.current.src = URL.createObjectURL(audioBlob);
      audioRef.current.play();
      setIsPlaying(true);
    }
  };
  
  // 暂停播放
  const pausePlaying = () => {
    if (audioRef.current) {
      audioRef.current.pause();
      setIsPlaying(false);
    }
  };
  
  // 删除录音
  const deleteRecording = () => {
    setAudioBlob(null);
    setDuration(0);
    onCancel();
  };
  
  // 发送录音
  const sendRecording = () => {
    if (audioBlob) {
      onSend(audioBlob, duration);
    }
  };
  
  // 格式化时间
  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };
  
  // 播放结束
  const handlePlayEnded = () => {
    setIsPlaying(false);
  };

  return (
    <div className="bg-bg-tertiary/80 backdrop-blur-lg rounded-lg p-4 border border-white/10 w-full">
      {/* 音频可视化 */}
      <div className="h-16 mb-4 bg-bg-primary/30 rounded-lg p-2 flex items-center justify-around">
        {visualData.map((value, index) => (
          <div 
            key={index}
            className="w-1 bg-accent-primary h-full rounded-full"
            style={{ 
              height: `${value * 100}%`, 
              opacity: isRecording && !isPaused ? 1 : 0.4,
              transform: isRecording && !isPaused ? 'scaleY(1)' : 'scaleY(0.6)',
              transition: 'all 0.1s ease-in-out'
            }}
          />
        ))}
      </div>
      
      {/* 计时器显示 */}
      <div className="flex justify-center mb-4">
        <div className="text-2xl font-mono text-text-primary">
          {formatTime(duration)}
        </div>
      </div>
      
      {/* 控制按钮 */}
      <div className="flex items-center justify-between">
        {!audioBlob ? (
          // 录音控制
          <>
            <Button 
              variant="outline" 
              className="w-10 h-10 rounded-full p-0 flex items-center justify-center"
              onClick={deleteRecording}
            >
              <Trash2 size={18} />
            </Button>
            
            {!isRecording ? (
              <Button 
                variant="primary"
                className="w-16 h-16 rounded-full p-0 flex items-center justify-center"
                onClick={startRecording}
              >
                <Mic size={24} />
              </Button>
            ) : (
              <Button 
                variant={isPaused ? "primary" : "outline"}
                className="w-16 h-16 rounded-full p-0 flex items-center justify-center"
                onClick={isPaused ? resumeRecording : pauseRecording}
              >
                {isPaused ? <Play size={24} /> : <Pause size={24} />}
              </Button>
            )}
            
            <Button 
              variant="outline" 
              className="w-10 h-10 rounded-full p-0 flex items-center justify-center"
              onClick={stopRecording}
              disabled={!isRecording}
            >
              <Square size={18} />
            </Button>
          </>
        ) : (
          // 播放控制
          <>
            <Button 
              variant="outline" 
              className="w-10 h-10 rounded-full p-0 flex items-center justify-center"
              onClick={deleteRecording}
            >
              <Trash2 size={18} />
            </Button>
            
            <Button 
              variant={isPlaying ? "outline" : "primary"}
              className="w-16 h-16 rounded-full p-0 flex items-center justify-center"
              onClick={isPlaying ? pausePlaying : playRecording}
            >
              {isPlaying ? <Pause size={24} /> : <Play size={24} />}
            </Button>
            
            <Button 
              variant="primary" 
              className="w-10 h-10 rounded-full p-0 flex items-center justify-center"
              onClick={sendRecording}
            >
              <Send size={18} />
            </Button>
          </>
        )}
      </div>
      
      {/* 隐藏的音频元素 */}
      <audio 
        ref={audioRef} 
        onEnded={handlePlayEnded}
        className="hidden"
      />
      
      <p className="text-xs text-text-secondary text-center mt-4">
        {isRecording 
          ? (isPaused ? '录音已暂停，点击继续' : '正在录音...') 
          : audioBlob 
            ? '录音完成，点击播放或发送' 
            : '点击麦克风开始录音'}
      </p>
    </div>
  );
};

export default VoiceMessageRecorder;