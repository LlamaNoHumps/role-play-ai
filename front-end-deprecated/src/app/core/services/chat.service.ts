import { Injectable } from '@angular/core';
import { Observable, concat, of, timer } from 'rxjs';
import { map, concatMap } from 'rxjs/operators';

@Injectable({ providedIn: 'root' })
export class ChatService {
  // 模拟把 LLM 回复分词流式返回
  streamReply(prompt: string): Observable<string> {
    const reply = `好的，我们来聊：${prompt}。我是你的 AI 角色，会用合适的语气继续对话。`;
    const tokens = reply.split('');
    return concat(...tokens.map((t, i) => timer(15*i).pipe(map(()=>t))));
  }
}
