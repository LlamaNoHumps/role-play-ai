export interface Message {
  id: string;
  sessionId: string;
  role: 'user' | 'assistant';
  content: string;
  createdAt: string;
}
export interface ChatSession {
  id: string;
  roleId: string;
  title: string;
  createdAt: string;
  updatedAt: string;
}
