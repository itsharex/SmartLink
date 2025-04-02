import type { NextApiRequest, NextApiResponse } from 'next';
import Groq from 'groq-sdk';

const groq = new Groq({ apiKey: process.env.GROQ_API_KEY });

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  const { text, fromLang, toLang } = req.body;

  try {
    const completion = await groq.chat.completions.create({
      model: 'mixtral-8x7b-32768',
      messages: [
        {
          role: 'system',
          content: `You are a translator. Translate the following text from ${fromLang} to ${toLang}. Provide only the translated text.`,
        },
        { role: 'user', content: text },
      ],
    });
    const translated = completion.choices[0]?.message?.content || text;
    res.status(200).json({ translated });
  } catch (error) {
    console.error('Translation error:', error);
    res.status(500).json({ error: 'Translation failed', original: text });
  }
}