import { NextResponse } from 'next/server';
import Groq from 'groq-sdk';
import { supportedLanguages } from '@/lib/constants/languages';

const groq = new Groq({ apiKey: process.env.GROQ_API_KEY });

const isValidTranslation = (translated: string, original: string): boolean => {
  if (translated === original) return false;

  const trimmed = translated.trim();
  if (trimmed.length < 2) return false;

  const punctuationRegex = /^[!,.?;:]+$/;
  if (punctuationRegex.test(trimmed)) return false;

  return true;
};

export async function POST(request: Request) {
  try {
    const { text, fromLang, toLang } = await request.json();

    // Validate required fields
    if (!text || !fromLang || !toLang) {
      return NextResponse.json(
        { error: 'Missing required fields: text, fromLang, toLang' },
        { status: 400 }
      );
    }

    // Validate language codes
    if (fromLang !== 'auto' && !supportedLanguages.includes(fromLang)) {
      return NextResponse.json(
        { error: `Unsupported source language: ${fromLang}` },
        { status: 400 }
      );
    }
    if (!supportedLanguages.includes(toLang)) {
      return NextResponse.json(
        { error: `Unsupported target language: ${toLang}` },
        { status: 400 }
      );
    }

    const completion = await groq.chat.completions.create({
      model: 'llama3-70b-8192',
      messages: [
        {
          role: 'system',
          content: `You are a professional translator. Your task is to translate the given text from ${fromLang === 'auto' ? 'its detected language' : fromLang} to ${toLang}. Ensure the translation is accurate and natural. If the text cannot be translated or the language is unsupported, return the original text unchanged. Provide only the translated text, without any additional commentary.`,
        },
        { role: 'user', content: text },
      ],
    });

    let translated = completion.choices[0]?.message?.content || text;

    if (!isValidTranslation(translated, text)) {
      console.warn(`Invalid translation for text "${text}" to ${toLang}: "${translated}"`);
      translated = text;
    }

    console.log(translated);

    return NextResponse.json({ translated }, { status: 200 });
  } catch (error) {
    console.error('Translation error:', error);
    return NextResponse.json(
      { error: 'Translation failed'},
      { status: 500 }
    );
  }
}