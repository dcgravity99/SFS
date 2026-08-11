import React, { useState } from 'react';
import { Globe, Moon, Sun } from 'lucide-react';

interface LocaleThemeSelectorProps {
  lang?: 'ta-IN' | 'en-US';
  onLocaleChange?: (locale: 'ta-IN' | 'en-US') => void;
}

export const LocaleThemeSelector: React.FC<LocaleThemeSelectorProps> = ({
  lang = 'ta-IN',
  onLocaleChange,
}) => {
  const [currentLocale, setCurrentLocale] = useState<'ta-IN' | 'en-US'>(lang);
  const [theme, setTheme] = useState<'Dark' | 'Light'>('Dark');

  const handleSelectLocale = (loc: 'ta-IN' | 'en-US') => {
    setCurrentLocale(loc);
    if (onLocaleChange) onLocaleChange(loc);
  };

  const titleText = currentLocale === 'ta-IN' ? 'மொழி & தோற்றம் (Locale & Theme)' : 'Language & Theme Settings';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Globe className="w-4 h-4 text-blue-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div>
          <label className="text-xs text-slate-300 block mb-2 font-semibold">Primary Product Language</label>
          <div className="grid grid-cols-2 gap-2">
            <button
              onClick={() => handleSelectLocale('ta-IN')}
              className={`py-2 px-3 text-xs font-semibold rounded-lg border transition-colors ${
                currentLocale === 'ta-IN'
                  ? 'bg-blue-500/20 text-blue-300 border-blue-500/40'
                  : 'bg-surface text-slate-400 border-border/60 hover:bg-surface-hover'
              }`}
            >
              தமிழ் (ta-IN)
            </button>

            <button
              onClick={() => handleSelectLocale('en-US')}
              className={`py-2 px-3 text-xs font-semibold rounded-lg border transition-colors ${
                currentLocale === 'en-US'
                  ? 'bg-blue-500/20 text-blue-300 border-blue-500/40'
                  : 'bg-surface text-slate-400 border-border/60 hover:bg-surface-hover'
              }`}
            >
              English (en-US)
            </button>
          </div>
        </div>

        <div>
          <label className="text-xs text-slate-300 block mb-2 font-semibold">Appearance Theme</label>
          <div className="grid grid-cols-2 gap-2">
            <button
              onClick={() => setTheme('Dark')}
              className={`py-2 px-3 text-xs font-semibold rounded-lg border transition-colors flex items-center justify-center gap-1.5 ${
                theme === 'Dark'
                  ? 'bg-purple-500/20 text-purple-300 border-purple-500/40'
                  : 'bg-surface text-slate-400 border-border/60'
              }`}
            >
              <Moon className="w-3.5 h-3.5" /> Dark Mode
            </button>

            <button
              onClick={() => setTheme('Light')}
              className={`py-2 px-3 text-xs font-semibold rounded-lg border transition-colors flex items-center justify-center gap-1.5 ${
                theme === 'Light'
                  ? 'bg-purple-500/20 text-purple-300 border-purple-500/40'
                  : 'bg-surface text-slate-400 border-border/60'
              }`}
            >
              <Sun className="w-3.5 h-3.5" /> Light Mode
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
