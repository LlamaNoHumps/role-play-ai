import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ChatService } from '../../../core/services/chat.service';


@Component({
  standalone: true,
  selector: 'app-chat',
  imports: [FormsModule],
  templateUrl: './chat.component.html',
  styleUrl: './chat.component.scss'
})
export class ChatComponent {
  input=''; streaming=false;
  messages: {role:'user'|'assistant', content:string}[] = [];

  constructor(private chat: ChatService) {}

  send(){
    const q = this.input.trim(); if (!q) return;
    this.messages.push({ role:'user', content: q });
    this.input=''; this.streaming=true;
    let acc=''; this.chat.streamReply(q).subscribe({
      next: t => { acc += t; this._patchAssistant(acc); },
      error: _ => { this.streaming=false; },
      complete: () => { this.streaming=false; }
    });
  }
  private _patchAssistant(text:string){
    const last = this.messages[this.messages.length-1];
    if (!last || last.role==='user') this.messages.push({role:'assistant', content: text});
    else last.content = text;
  }
}
