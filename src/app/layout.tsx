import { AuthProvider } from '@/context/AuthContext';
import './globals.css';
import { ContactsProvider } from '@/context/ContactsContext';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <AuthProvider>
          <ContactsProvider>
            {children}
          </ContactsProvider>
        </AuthProvider>
      </body>
    </html>
  );
}