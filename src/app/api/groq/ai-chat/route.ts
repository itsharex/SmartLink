// src/app/api/groqChat/route.ts
import { NextResponse } from 'next/server';
import Groq from 'groq-sdk';

const groq = new Groq({ apiKey: process.env.GROQ_API_KEY });

export async function POST(request: Request) {
  try {
    const { messages } = await request.json();
    
    // Call the Groq API in streaming mode.
    const streamIterator = await groq.chat.completions.create({
      messages,
      model: "llama3-70b-8192",
      temperature: 0.5,
      max_completion_tokens: 1024,
      top_p: 1,
      stop: null,
      stream: true,
    });
    
    const encoder = new TextEncoder();
    const stream = new ReadableStream({
      async start(controller) {
        try {
          for await (const chunk of streamIterator) {
            const text = chunk.choices[0]?.delta?.content || "";
            controller.enqueue(encoder.encode(text));
          }
        } catch (err) {
          console.error("Stream error:", err);
          controller.error(err);
        } finally {
          controller.close();
        }
      }
    });
    
    return new Response(stream, {
      headers: { "Content-Type": "text/plain" },
    });
  } catch (error) {
    console.error("Error calling Groq API:", error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
