import React, { useState } from 'react';
import { MessageSquare, Send } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { ReviewCommentView } from './types';

interface ReviewThreadPanelProps {
  comments?: ReviewCommentView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultComments: ReviewCommentView[] = [
  {
    comment_id: 'cmt-01',
    artist_id: 'artist-dir-ag',
    artist_name: 'AG (Director)',
    timecode_frame: 48,
    content: { 'ta-IN': 'இந்தக் கோணத்தில் வெளிச்சம் இன்னும் கூர்மையாக இருக்க வேண்டும்.', 'en-US': 'Key light should be sharper on lead actor.' },
    created_at: '2026-08-04T08:30:00Z',
  },
];

export const ReviewThreadPanel: React.FC<ReviewThreadPanelProps> = ({
  comments = defaultComments,
  lang = 'ta-IN',
}) => {
  const [newComment, setNewComment] = useState('');
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleAddComment = async () => {
    if (!newComment.trim()) return;

    const response = await StudioIpcService.executeEngineCommand('collaboration_create_review', {
      shot_id: 'shot-01-wide',
      timecode_frame: 48,
      comment: newComment,
    });

    if (response.success) {
      setNewComment('');
      setStatusMessage(lang === 'ta-IN' ? 'கருத்து சேமிக்கப்பட்டது' : 'Review Note Added via IPC');
    }
  };

  const titleText = lang === 'ta-IN' ? 'மதிப்பாய்வு உரையாடல் (Review Threads)' : 'Shot Review Annotations';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <MessageSquare className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1 mb-3">
        {comments.map((c) => {
          const text = c.content[lang] || c.content['en-US'];
          return (
            <div key={c.comment_id} className="bg-surface/80 border border-border/60 rounded-lg p-3">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-slate-200">{c.artist_name}</span>
                <span className="text-[10px] font-mono text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded">
                  Frame #{c.timecode_frame}
                </span>
              </div>
              <p className="text-xs text-slate-300 mt-2">{text}</p>
            </div>
          );
        })}
      </div>

      <div className="flex gap-2 pt-2 border-t border-border/40">
        <input
          type="text"
          value={newComment}
          onChange={(e) => setNewComment(e.target.value)}
          placeholder={lang === 'ta-IN' ? 'மதிப்பாய்வு கருத்து...' : 'Enter review note...'}
          className="flex-1 bg-surface border border-border/60 rounded p-2 text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-purple-500"
        />
        <button
          onClick={handleAddComment}
          className="p-2 bg-purple-600 hover:bg-purple-500 text-white rounded transition-colors"
        >
          <Send className="w-4 h-4" />
        </button>
      </div>

      {statusMessage && (
        <div className="p-2 mt-2 bg-purple-500/10 border border-purple-500/20 rounded text-[10px] font-mono text-purple-300">
          {statusMessage}
        </div>
      )}
    </div>
  );
};
