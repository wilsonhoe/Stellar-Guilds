
import { useState, useRef, useEffect } from "react";
import type { Tab, TabsProps } from "../../types/ui";

export function Tabs({ tabs, defaultTab, className = "" }: TabsProps) {
  const [activeTab, setActiveTab] = useState(defaultTab ?? tabs[0]?.id);
  const [underlineStyle, setUnderlineStyle] = useState({ left: 0, width: 0 });
  const tabRefs = useRef<Record<string, HTMLButtonElement | null>>({});

  useEffect(() => {
    const el = tabRefs.current[activeTab];
    if (!el) return;
    setUnderlineStyle({ left: el.offsetLeft, width: el.offsetWidth });
  }, [activeTab]);

  useEffect(() => {
    const el = tabRefs.current[activeTab];
    if (!el) return;
    setUnderlineStyle({ left: el.offsetLeft, width: el.offsetWidth });
  }, []); 

  const activeContent = tabs.find((t) => t.id === activeTab)?.content;

  return (
    <div
      className={`bg-[#0d0f1a] text-[#c8cce8] rounded-2xl overflow-hidden shadow-[0_0_0_1px_rgba(255,255,255,0.07),0_24px_64px_rgba(0,0,0,0.6)] ${className}`}
    >
      {/* Tab bar */}
      <div
        role="tablist"
        className="relative flex bg-[#13162a] border-b border-white/[0.07] px-2"
      >
        {tabs.map((tab) => (
          <button
            key={tab.id}
            ref={(el) => { tabRefs.current[tab.id] = el; }}
            role="tab"
            aria-selected={activeTab === tab.id}
            aria-controls={`tabpanel-${tab.id}`}
            id={`tab-${tab.id}`}
            onClick={() => setActiveTab(tab.id)}
            className={[
              "relative z-10 inline-flex items-center gap-1.5 px-5 py-3.5",
              "bg-transparent border-none cursor-pointer whitespace-nowrap",
              "font-medium text-sm tracking-wide transition-colors duration-200",
              activeTab === tab.id
                ? "text-white"
                : "text-[#6b7194] hover:text-[#c8cce8]",
            ].join(" ")}
          >
            {tab.icon && (
              <span
                className={`flex items-center text-base transition-opacity duration-200 ${
                  activeTab === tab.id ? "opacity-100" : "opacity-75"
                }`}
              >
                {tab.icon}
              </span>
            )}
            {tab.label}
          </button>
        ))}

        {/* Animated underline */}
        <span
          aria-hidden
          className="absolute bottom-[-1px] h-0.5 bg-[#7c6bff] rounded-t pointer-events-none shadow-[0_0_12px_rgba(124,107,255,0.35)]"
          style={{
            left: underlineStyle.left,
            width: underlineStyle.width,
            transition:
              "left 0.3s cubic-bezier(0.4,0,0.2,1), width 0.3s cubic-bezier(0.4,0,0.2,1)",
          }}
        />
      </div>

      {/* Content panel */}
      <div
        key={activeTab}
        role="tabpanel"
        id={`tabpanel-${activeTab}`}
        aria-labelledby={`tab-${activeTab}`}
        className="p-7 animate-[panelIn_0.22s_ease_both]"
      >
        {activeContent}
      </div>

      <style>{`
        @keyframes panelIn {
          from { opacity: 0; transform: translateY(6px); }
          to   { opacity: 1; transform: translateY(0); }
        }
      `}</style>
    </div>
  );
}

export default Tabs;