import React from 'react';
import { ShieldCheck, CheckCircle } from 'lucide-react';
import { SecurityAuditEventView } from './types';

interface SecurityAuditPanelProps {
  audits?: SecurityAuditEventView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultAudits: SecurityAuditEventView[] = [
  { audit_id: 'aud-asvs-l2', standard_name: 'OWASP ASVS Level 2 Security Architecture', status: 'Compliant', verified_at: '2026-08-04T08:00:00Z' },
  { audit_id: 'aud-csp-strict', standard_name: 'Strict Content Security Policy (No inline/eval)', status: 'Compliant', verified_at: '2026-08-04T08:00:00Z' },
  { audit_id: 'aud-assetid-handles', standard_name: 'AssetId Handle Integrity & Cryptographic Checksums', status: 'Compliant', verified_at: '2026-08-04T08:00:00Z' },
];

export const SecurityAuditPanel: React.FC<SecurityAuditPanelProps> = ({
  audits = defaultAudits,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'பாதுகாப்புத் தணிக்கை (Security Audit Trail)' : 'Security & Compliance Audit Panel';

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <ShieldCheck className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3">
        {audits.map((a) => (
          <div key={a.audit_id} className="p-3 bg-surface/80 border border-border/60 rounded-lg flex items-center justify-between">
            <div>
              <div className="text-xs font-semibold text-slate-200">{a.standard_name}</div>
              <div className="text-[10px] font-mono text-slate-400 mt-0.5">{a.audit_id}</div>
            </div>
            <div className="text-xs font-mono text-emerald-400 bg-emerald-500/10 px-2.5 py-1 rounded border border-emerald-500/20 flex items-center gap-1">
              <CheckCircle className="w-3 h-3" /> {a.status}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
