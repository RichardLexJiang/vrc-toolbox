import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef, useState, type FormEvent, type ReactNode } from "react";
import "./App.css";

type Page = "overview" | "chatbox" | "avatar" | "input" | "settings";
type Theme = "light" | "dark";
type Settings = { oscTarget: string };
type ChatboxHistoryItem = {
  originContent: string;
  parsedContent: string | null;
  sendTime: number;
};
type ChatboxTemplate = { name: string; content: string };
type ChatboxRepeatStatus = {
  active: boolean;
  messages: string[] | null;
  error: string | null;
};
type EmojiCategory = { label: string; emojis: readonly string[] };

const emojiCategories: EmojiCategory[] = [
  { label: "Smileys", emojis: ["😀", "😃", "😄", "😁", "😆", "😅", "😂", "🤣", "😊", "😇", "🙂", "🙃", "😉", "😍", "🥰", "😘", "😋", "😎", "🤩", "🥳", "🤔", "🤭", "😴", "😭", "😢", "😤", "🤯", "🥺", "😳", "🫠"] },
  { label: "Hands", emojis: ["👋", "🤚", "✋", "🖖", "👌", "🤌", "🤏", "✌️", "🤞", "🫰", "🤟", "🤘", "🤙", "👈", "👉", "👆", "👇", "☝️", "👍", "👎", "✊", "👊", "👏", "🙌", "👐", "🤝", "🙏", "💪", "🫶", "💅"] },
  { label: "Hearts", emojis: ["❤️", "🧡", "💛", "💚", "💙", "💜", "🖤", "🤍", "🤎", "💔", "❣️", "💕", "💞", "💓", "💗", "💖", "💘", "💝", "💟", "♥️", "💌", "💋"] },
  { label: "Celebration", emojis: ["🎉", "🎊", "🎈", "🎂", "🎁", "🎀", "🎆", "🎇", "✨", "🌟", "⭐", "💫", "🔥", "💥", "🎵", "🎶", "🎤", "🎧", "🎮", "🕹️", "🏆", "🥇", "🚀", "🪄"] },
  { label: "Animals", emojis: ["🐶", "🐱", "🐰", "🦊", "🐻", "🐼", "🐸", "🐵", "🐧", "🐦", "🦄", "🐝", "🦋", "🐢", "🐬", "🐳", "🦈", "🦖", "🐉", "🐾", "🐣", "🦉", "🦇", "🪼"] },
  { label: "Nature", emojis: ["🌸", "🌹", "🌻", "🌼", "🌷", "🪻", "🌱", "🌿", "🍀", "🌵", "🌴", "🍁", "🍂", "🍄", "🌈", "☀️", "🌙", "🌛", "⭐", "☁️", "❄️", "⚡", "🌊", "💧"] },
  { label: "Food", emojis: ["🍓", "🍒", "🍑", "🍉", "🍇", "🍍", "🥑", "🍕", "🍔", "🍟", "🌮", "🍜", "🍣", "🍩", "🍪", "🍰", "🧁", "🍫", "☕", "🧋", "🍵", "🥤", "🍿", "🧀"] },
  { label: "Symbols", emojis: ["💬", "💭", "💤", "💯", "✅", "❌", "⚠️", "❓", "❗", "💡", "🔔", "🔒", "🔑", "🎵", "➕", "➖", "♾️", "🔆", "🔅", "💠", "🔮", "🪩", "🎐", "🧸"] },
];
type ConnectionStatus = "unknown" | "connected" | "noConnection";
type IconName =
  | "grid"
  | "message"
  | "user"
  | "gamepad"
  | "settings"
  | "search"
  | "arrow"
  | "sparkles"
  | "chevron"
  | "send"
  | "check"
  | "clock"
  | "external"
  | "activity"
  | "wifi"
  | "keyboard"
  | "sliders"
  | "shield"
  | "moon"
  | "sun";

const iconPaths: Record<IconName, ReactNode> = {
  grid: <><rect x="3.5" y="3.5" width="7" height="7" rx="1.5" /><rect x="13.5" y="3.5" width="7" height="7" rx="1.5" /><rect x="3.5" y="13.5" width="7" height="7" rx="1.5" /><rect x="13.5" y="13.5" width="7" height="7" rx="1.5" /></>,
  message: <><path d="M20 11.5a7.5 7.5 0 0 1-7.5 7.5H5l-1 2.5L7 19a7.5 7.5 0 1 1 13-7.5Z" /><path d="M8 11h.01M12 11h.01M16 11h.01" /></>,
  user: <><circle cx="12" cy="8" r="3.5" /><path d="M5 20a7 7 0 0 1 14 0" /></>,
  gamepad: <><path d="M6.5 9h11a4.5 4.5 0 0 1 4.2 6.1l-1 2.5a2 2 0 0 1-3.1.8L14.5 16h-5l-3.1 2.4a2 2 0 0 1-3.1-.8l-1-2.5A4.5 4.5 0 0 1 6.5 9Z" /><path d="M7 12v4M5 14h4M16.5 13.5h.01M18.5 15.5h.01" /></>,
  settings: <><path d="M12 8.5a3.5 3.5 0 1 0 0 7 3.5 3.5 0 0 0 0-7Z" /><path d="m19.4 15 .1.1a1.8 1.8 0 0 1-2.5 2.5l-.1-.1a1.8 1.8 0 0 0-3 .9v.2a1.8 1.8 0 0 1-3.6 0v-.2a1.8 1.8 0 0 0-3-.9l-.1.1a1.8 1.8 0 0 1-2.5-2.5l.1-.1a1.8 1.8 0 0 0-.9-3H3.7a1.8 1.8 0 0 1 0-3.6h.2a1.8 1.8 0 0 0 .9-3l-.1-.1a1.8 1.8 0 0 1 2.5-2.5l.1.1a1.8 1.8 0 0 0 3-.9v-.2a1.8 1.8 0 0 1 3.6 0v.2a1.8 1.8 0 0 0 3 .9l.1-.1a1.8 1.8 0 0 1 2.5 2.5l-.1.1a1.8 1.8 0 0 0 .9 3h.2a1.8 1.8 0 0 1 0 3.6h-.2a1.8 1.8 0 0 0-.9 3Z" /></>,
  search: <><circle cx="10.8" cy="10.8" r="6.8" /><path d="m16 16 4.5 4.5" /></>,
  arrow: <><path d="M5 12h14M13 6l6 6-6 6" /></>,
  sparkles: <><path d="m12 3 1.7 5.3L19 10l-5.3 1.7L12 17l-1.7-5.3L5 10l5.3-1.7L12 3Z" /><path d="m19 15 .8 2.2L22 18l-2.2.8L19 21l-.8-2.2L16 18l2.2-.8L19 15ZM5 3l.7 2.3L8 6l-2.3.7L5 9l-.7-2.3L2 6l2.3-.7L5 3Z" /></>,
  chevron: <path d="m9 18 6-6-6-6" />,
  send: <><path d="m21 3-7.5 18-3.7-7.8L2 9.5 21 3Z" /><path d="M9.8 13.2 14 9" /></>,
  check: <path d="m5 12 4 4L19 6" />,
  clock: <><circle cx="12" cy="12" r="9" /><path d="M12 7v5l3 2" /></>,
  external: <><path d="M14 4h6v6M20 4l-9 9" /><path d="M18 13v6a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h6" /></>,
  activity: <path d="M3 12h4l3-8 4 16 3-8h4" />,
  wifi: <><path d="M5 9a11 11 0 0 1 14 0M8 12a6.5 6.5 0 0 1 8 0M11 15a2 2 0 0 1 2 0" /><path d="M12 19h.01" /></>,
  keyboard: <><rect x="3" y="5" width="18" height="14" rx="2" /><path d="M7 9h.01M10 9h.01M13 9h.01M16 9h.01M7 12h.01M10 12h.01M13 12h.01M16 12h.01M8 15h8" /></>,
  sliders: <><path d="M4 7h9M17 7h3M4 17h3M11 17h9" /><circle cx="15" cy="7" r="2" /><circle cx="9" cy="17" r="2" /></>,
  shield: <><path d="M12 22s8-4 8-11V5l-8-3-8 3v6c0 7 8 11 8 11Z" /><path d="m9 12 2 2 4-4" /></>,
  moon: <path d="M20.5 14.5A8.5 8.5 0 0 1 9.5 3.5 8.5 8.5 0 1 0 20.5 14.5Z" />,
  sun: <><circle cx="12" cy="12" r="4" /><path d="M12 2v2M12 20v2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M2 12h2m16 0h2M4.93 19.07l1.42-1.42m11.3-11.3 1.42-1.42" /></>,
};

function Icon({ name, size = 20 }: { name: IconName; size?: number }) {
  return (
    <svg
      aria-hidden="true"
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.7"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      {iconPaths[name]}
    </svg>
  );
}

function connectionStatusLabel(status: ConnectionStatus) {
  switch (status) {
    case "connected":
      return "Connected";
    case "noConnection":
      return "Not connected";
    case "unknown":
      return "Unknown";
  }
}

function getChatboxHistory() {
  return invoke<ChatboxHistoryItem[]>("get_chatbox_history");
}

function getChatboxTemplates() {
  return invoke<ChatboxTemplate[]>("get_template_list");
}

function errorMessage(error: unknown) {
  if (error instanceof Error) return error.message;
  if (typeof error === "object" && error !== null && "message" in error && typeof error.message === "string") {
    return error.message;
  }
  return String(error);
}

const tools: { id: Exclude<Page, "overview" | "settings">; title: string; description: string; icon: IconName; state: "Ready" | "Coming soon"; accent: string }[] = [
  { id: "chatbox", title: "Chatbox", description: "Send a message to your VRChat chatbox.", icon: "message", state: "Ready", accent: "lilac" },
  { id: "avatar", title: "Avatar", description: "Adjust your avatar’s eye height through OSC.", icon: "user", state: "Ready", accent: "peach" },
  { id: "input", title: "Input controls", description: "Control supported VRChat inputs.", icon: "gamepad", state: "Coming soon", accent: "blue" },
];

const themeStorageKey = "vrc-toolbox-theme";

const pageDetails: Record<Page, { title: string; eyebrow: string }> = {
  overview: { title: "Overview", eyebrow: "YOUR WORKSPACE" },
  chatbox: { title: "Chatbox", eyebrow: "BUILT-IN TOOL" },
  avatar: { title: "Avatar controls", eyebrow: "BUILT-IN TOOL" },
  input: { title: "Input controls", eyebrow: "BUILT-IN TOOL" },
  settings: { title: "Settings", eyebrow: "PREFERENCES" },
};

function App() {
  const [page, setPage] = useState<Page>("overview");
  const [theme, setTheme] = useState<Theme>(() =>
    window.localStorage.getItem(themeStorageKey) === "light" ? "light" : "dark",
  );
  const [message, setMessage] = useState("");
  const [isSending, setIsSending] = useState(false);
  const [chatboxHistory, setChatboxHistory] = useState<ChatboxHistoryItem[]>([]);
  const [chatboxTemplates, setChatboxTemplates] = useState<ChatboxTemplate[]>([]);
  const [isHistoryLoading, setIsHistoryLoading] = useState(false);
  const [historyError, setHistoryError] = useState("");
  const [isTemplatesLoading, setIsTemplatesLoading] = useState(false);
  const [templatesError, setTemplatesError] = useState("");
  const [isTemplateSaving, setIsTemplateSaving] = useState(false);
  const [repeatStatus, setRepeatStatus] = useState<ChatboxRepeatStatus>({
    active: false,
    messages: null,
    error: null,
  });
  const [notice, setNotice] = useState("");
  const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>("unknown");
  const [oscTarget, setOscTarget] = useState("127.0.0.1:9000");
  const [savedTarget, setSavedTarget] = useState("127.0.0.1:9000");

  useEffect(() => {
    window.localStorage.setItem(themeStorageKey, theme);
  }, [theme]);

  function showNotice(text: string) {
    setNotice(text);
    window.setTimeout(() => setNotice(""), 3800);
  }

  useEffect(() => {
    if (!isTauri()) return;

    let cancelled = false;
    invoke<Settings>("get_local_settings")
      .then((settings) => {
        if (cancelled) return;
        setOscTarget(settings.oscTarget);
        setSavedTarget(settings.oscTarget);
      })
      .catch((error: unknown) => {
        if (!cancelled) showNotice(`Could not load settings: ${String(error)}`);
      });
    invoke<ConnectionStatus>("get_connection_status")
      .then((status) => {
        if (!cancelled) setConnectionStatus(status);
      })
      .catch((error: unknown) => {
        if (!cancelled) showNotice(`Could not load connection status: ${String(error)}`);
      });

    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!isTauri()) return;

    let cancelled = false;
    let unlisten: (() => void) | undefined;
    listen<ChatboxRepeatStatus>("chatbox-repeat-status", (event) => {
      setRepeatStatus(event.payload);
      if (event.payload.error) showNotice(`Repeating stopped: ${event.payload.error}`);
    }).then((dispose) => {
      if (cancelled) {
        dispose();
        return;
      }
      unlisten = dispose;
      invoke<ChatboxRepeatStatus>("get_chatbox_repeat_status")
        .then((status) => {
          if (!cancelled) setRepeatStatus(status);
        })
        .catch((error: unknown) => {
          if (!cancelled) showNotice(`Could not load repeat status: ${String(error)}`);
        });
    }).catch((error: unknown) => {
      if (!cancelled) showNotice(`Could not listen for repeat status: ${String(error)}`);
    });

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);

  useEffect(() => {
    if (page !== "chatbox" || !isTauri()) return;

    let cancelled = false;
    setIsHistoryLoading(true);
    setHistoryError("");
    setIsTemplatesLoading(true);
    setTemplatesError("");
    getChatboxHistory()
      .then((history) => {
        if (!cancelled) setChatboxHistory(history);
      })
      .catch((error: unknown) => {
        if (!cancelled) setHistoryError(`Could not load chatbox history: ${String(error)}`);
      })
      .finally(() => {
        if (!cancelled) setIsHistoryLoading(false);
      });
    getChatboxTemplates()
      .then((templates) => {
        if (!cancelled) setChatboxTemplates(templates);
      })
      .catch((error: unknown) => {
        if (!cancelled) setTemplatesError(`Could not load templates: ${String(error)}`);
      })
      .finally(() => {
        if (!cancelled) setIsTemplatesLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, [page]);

  async function submitMessage(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const trimmedMessage = message.trim();
    if (!trimmedMessage) return;
    if (!isTauri()) {
      showNotice("Sending is only available in the VRC Toolbox desktop app.");
      return;
    }

    setIsSending(true);
    try {
      await invoke("send_message", { message: trimmedMessage, appendHis: true });
      setMessage("");
      showNotice("Message sent to your VRChat chatbox.");
      try {
        setChatboxHistory(await getChatboxHistory());
        setHistoryError("");
      } catch (error) {
        setHistoryError(`Could not refresh chatbox history: ${String(error)}`);
      }
    } catch (error) {
      const detail =
        error instanceof Error
          ? error.message
          : typeof error === "object" && error !== null && "message" in error && typeof error.message === "string"
            ? error.message
            : String(error);
      showNotice(`Could not send message: ${detail}`);
    } finally {
      setIsSending(false);
    }
  }

  async function saveTemplate(name: string, content: string) {
    const trimmedName = name.trim();
    const trimmedContent = content.trim();
    if (!trimmedContent) return;
    if (!trimmedName || trimmedName.length > 40) {
      showNotice("Template names must be between 1 and 40 characters.");
      return;
    }
    if (chatboxTemplates.some((template) => template.name.toLowerCase() === trimmedName.toLowerCase())) {
      showNotice("A template with that name already exists.");
      return;
    }
    if (chatboxTemplates.length >= 20) {
      showNotice("You can save up to 20 templates.");
      return;
    }

    const updatedTemplates = [...chatboxTemplates, { name: trimmedName, content: trimmedContent }];
    setIsTemplateSaving(true);
    try {
      await invoke("write_template_list", { list: updatedTemplates });
      setChatboxTemplates(updatedTemplates);
      setTemplatesError("");
      showNotice("Template saved.");
    } catch (error) {
      showNotice(`Could not save template: ${errorMessage(error)}`);
    } finally {
      setIsTemplateSaving(false);
    }
  }

  async function deleteTemplate(index: number) {
    const updatedTemplates = chatboxTemplates.filter((_, templateIndex) => templateIndex !== index);
    setIsTemplateSaving(true);
    try {
      await invoke("write_template_list", { list: updatedTemplates });
      setChatboxTemplates(updatedTemplates);
      setTemplatesError("");
      showNotice("Template deleted.");
    } catch (error) {
      showNotice(`Could not delete template: ${errorMessage(error)}`);
    } finally {
      setIsTemplateSaving(false);
    }
  }

  async function updateTemplate(index: number, name: string, content: string): Promise<boolean> {
    const trimmedName = name.trim();
    const trimmedContent = content.trim();
    if (!trimmedName || trimmedName.length > 40) {
      showNotice("Template names must be between 1 and 40 characters.");
      return false;
    }
    if (!trimmedContent || trimmedContent.length > 144) {
      showNotice("Template content must be between 1 and 144 characters.");
      return false;
    }
    if (chatboxTemplates.some((template, templateIndex) => templateIndex !== index && template.name.toLowerCase() === trimmedName.toLowerCase())) {
      showNotice("A template with that name already exists.");
      return false;
    }
    const updatedTemplates = chatboxTemplates.map((template, templateIndex) =>
      templateIndex === index ? { name: trimmedName, content: trimmedContent } : template,
    );
    setIsTemplateSaving(true);
    try {
      await invoke("write_template_list", { list: updatedTemplates });
      setChatboxTemplates(updatedTemplates);
      showNotice("Template updated.");
      return true;
    } catch (error) {
      showNotice(`Could not update template: ${errorMessage(error)}`);
      return false;
    } finally {
      setIsTemplateSaving(false);
    }
  }

  async function startRepeating(messages: string[]) {
    try {
      const status = await invoke<ChatboxRepeatStatus>("start_chatbox_repeat", {
        message: messages,
      });
      setRepeatStatus(status);
      showNotice("Repeating messages started.");
    } catch (error) {
      showNotice(`Could not start repeating message: ${errorMessage(error)}`);
    }
  }

  async function stopRepeating() {
    try {
      const status = await invoke<ChatboxRepeatStatus>("stop_chatbox_repeat");
      setRepeatStatus(status);
      showNotice("Repeating message stopped.");
    } catch (error) {
      showNotice(`Could not stop repeating message: ${errorMessage(error)}`);
    }
  }

  async function saveSettings(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const settings = { oscTarget: oscTarget.trim() };
    if (isTauri()) {
      try {
        await invoke("save_local_settings", { settings });
      } catch (error) {
        showNotice(`Could not save settings: ${String(error)}`);
        return;
      }
    }
    setOscTarget(settings.oscTarget);
    setSavedTarget(settings.oscTarget);
    showNotice(isTauri() ? "Settings saved on this device." : "Target updated for this preview.");
  }

  const currentPage = pageDetails[page];

  return (
    <div className="app-shell" data-theme={theme}>
      <aside className="sidebar">
        <button className="brand" onClick={() => setPage("overview")} aria-label="VRC Toolbox home">
          <span className="brand-mark"><span /><span /><span /></span>
          <span className="brand-copy"><strong>vrc<span>.</span>toolbox</strong><small>YOUR VRCHAT COMPANION</small></span>
        </button>

        <nav className="primary-nav" aria-label="Main navigation">
          <NavItem icon="grid" label="Overview" active={page === "overview"} onClick={() => setPage("overview")} />
          {tools.map((tool) => (
            <NavItem key={tool.id} icon={tool.icon} label={tool.title} active={page === tool.id} onClick={() => setPage(tool.id)} />
          ))}
          <NavItem icon="settings" label="Settings" active={page === "settings"} onClick={() => setPage("settings")} />
        </nav>
      </aside>

      <main className="main-panel">
        <header className="topbar">
          <div className="breadcrumbs"><span>Workspace</span><Icon name="chevron" size={14} /><strong>{currentPage.title}</strong></div>
          <div className="topbar-actions">
            <button
              className="theme-toggle"
              type="button"
              onClick={() => setTheme((currentTheme) => currentTheme === "dark" ? "light" : "dark")}
              aria-label={`Switch to ${theme === "dark" ? "light" : "dark"} theme`}
              title={`Switch to ${theme === "dark" ? "light" : "dark"} theme`}
            >
              <Icon name={theme === "dark" ? "sun" : "moon"} size={17} />
            </button>
            <button className="connection-pill" onClick={() => setPage("settings")} title="Open connection settings">
              <span className={`status-dot status-${connectionStatus}`} /> <span>OSC {connectionStatusLabel(connectionStatus).toLowerCase()}</span>
            </button>
          </div>
        </header>

        <div className="page-content">
          {page === "overview" && (
            <Overview
              onOpenTool={setPage}
            />
          )}
          {page === "chatbox" && (
            <ChatboxPage
              message={message}
              setMessage={setMessage}
              onSubmit={submitMessage}
              isSending={isSending}
              isDesktop={isTauri()}
              history={chatboxHistory}
              isHistoryLoading={isHistoryLoading}
              historyError={historyError}
              templates={chatboxTemplates}
              isTemplatesLoading={isTemplatesLoading}
              templatesError={templatesError}
              isTemplateSaving={isTemplateSaving}
              repeatStatus={repeatStatus}
              onSaveTemplate={saveTemplate}
              onDeleteTemplate={deleteTemplate}
              onUpdateTemplate={updateTemplate}
              onStartRepeat={startRepeating}
              onStopRepeat={stopRepeating}
            />
          )}
          {page === "avatar" && <AvatarPage isDesktop={isTauri()} />}
          {page === "input" && (
            <ComingSoonPage onBack={() => setPage("overview")} />
          )}
          {page === "settings" && (
            <SettingsPage
              target={oscTarget}
              setTarget={setOscTarget}
              onSubmit={saveSettings}
              savedTarget={savedTarget}
              connectionStatus={connectionStatus}
            />
          )}
        </div>
      </main>
      {notice && <div className="toast" role="status"><span className="toast-icon"><Icon name="check" size={16} /></span>{notice}</div>}
    </div>
  );
}

function NavItem({ icon, label, active, onClick }: { icon: IconName; label: string; active: boolean; onClick: () => void }) {
  return (
    <button className={`nav-item ${active ? "active" : ""}`} onClick={onClick}>
      <Icon name={icon} size={18} /><span>{label}</span>{active && <span className="nav-active-mark" />}
    </button>
  );
}

function Overview({
  onOpenTool,
}: {
  onOpenTool: (page: Page) => void;
}) {
  return (
    <section className="overview-page" aria-labelledby="overview-heading">
      <div className="overview-heading">
        <h1 id="overview-heading">Your VRChat tools</h1>
        <p>Choose a tool to get started.</p>
      </div>
      <div className="tool-grid">
        {tools.map((tool) => (
          <button
            className={`tool-card ${tool.accent} ${tool.state === "Coming soon" ? "tool-card-soon" : ""}`}
            key={tool.id}
            onClick={() => onOpenTool(tool.id)}
          >
            <div className="tool-card-top">
              <span className="tool-icon"><Icon name={tool.icon} size={20} /></span>
              <span className={`tool-state ${tool.state === "Coming soon" ? "state-soon" : ""}`}>
                <span />{tool.state}
              </span>
            </div>
            <div className="tool-card-title">{tool.title}</div>
            <p>{tool.description}</p>
            <div className="tool-card-bottom">
              <span>{tool.state === "Ready" ? "Open tool" : "Not available yet"}</span>
              <Icon name="arrow" size={16} />
            </div>
          </button>
        ))}
      </div>
    </section>
  );
}

function ChatboxPage({
  message,
  setMessage,
  onSubmit,
  isSending,
  isDesktop,
  history,
  isHistoryLoading,
  historyError,
  templates,
  isTemplatesLoading,
  templatesError,
  isTemplateSaving,
  repeatStatus,
  onSaveTemplate,
  onDeleteTemplate,
  onUpdateTemplate,
  onStartRepeat,
  onStopRepeat,
}: {
  message: string;
  setMessage: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  isSending: boolean;
  isDesktop: boolean;
  history: ChatboxHistoryItem[];
  isHistoryLoading: boolean;
  historyError: string;
  templates: ChatboxTemplate[];
  isTemplatesLoading: boolean;
  templatesError: string;
  isTemplateSaving: boolean;
  repeatStatus: ChatboxRepeatStatus;
  onSaveTemplate: (name: string, content: string) => void;
  onDeleteTemplate: (index: number) => void;
  onUpdateTemplate: (index: number, name: string, content: string) => Promise<boolean>;
  onStartRepeat: (messages: string[]) => void;
  onStopRepeat: () => void;
}) {
  const [isEmojiPickerOpen, setIsEmojiPickerOpen] = useState(false);
  const [emojiCategory, setEmojiCategory] = useState(emojiCategories[0].label);
  const [listTab, setListTab] = useState<"templates" | "history">("templates");
  const [selectedRepeatIndices, setSelectedRepeatIndices] = useState<Set<number>>(() => new Set());
  const [isSaveTemplateFormOpen, setIsSaveTemplateFormOpen] = useState(false);
  const [templateNameDraft, setTemplateNameDraft] = useState("");
  const [editIndex, setEditIndex] = useState<number | null>(null);
  const [editNameDraft, setEditNameDraft] = useState("");
  const [editContentDraft, setEditContentDraft] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const activeEmojiCategory = emojiCategories.find((category) => category.label === emojiCategory) ?? emojiCategories[0];

  useEffect(() => {
    setSelectedRepeatIndices(new Set());
  }, [templates]);

  const selectedRepeatMessages = [...selectedRepeatIndices]
    .sort((left, right) => left - right)
    .map((index) => templates[index]?.content)
    .filter((content): content is string => content !== undefined);

  function insertEmoji(emoji: string) {
    const textarea = textareaRef.current;
    if (!textarea) return;

    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const availableLength = 144 - (textarea.value.length - (end - start));
    let insertedEmoji = "";
    for (const character of emoji) {
      if (insertedEmoji.length + character.length > availableLength) break;
      insertedEmoji += character;
    }
    if (!insertedEmoji) return;

    setMessage(`${textarea.value.slice(0, start)}${insertedEmoji}${textarea.value.slice(end)}`);
    window.requestAnimationFrame(() => {
      textarea.focus();
      const cursorPosition = start + insertedEmoji.length;
      textarea.setSelectionRange(cursorPosition, cursorPosition);
    });
  }

  return (
    <>
      <PageIntro eyebrow="BUILT-IN TOOL · 01" title="Chatbox" description="Compose a quick note, revisit a recent message, and send it straight to your VRChat chatbox." />
      <div className="chat-layout">
        <section className="content-card composer-card">
          <div className="content-card-header"><span className="section-icon lilac-icon"><Icon name="message" size={19} /></span><span><strong>Compose a message</strong><small>Keep it short and unmistakably you</small></span><span className="preview-label"><span /> LIVE PREVIEW</span></div>
          <form onSubmit={onSubmit}>
            <label className="field-label" htmlFor="chat-message">YOUR MESSAGE <span>UP TO 144 CHARACTERS</span></label>
            <textarea
              id="chat-message"
              ref={textareaRef}
              value={message}
              onChange={(event) => setMessage(event.currentTarget.value.slice(0, 144))}
              onKeyDown={(event) => {
                if (event.key === "Enter" && event.shiftKey && !event.nativeEvent.isComposing) {
                  event.preventDefault();
                  event.currentTarget.form?.requestSubmit();
                }
              }}
              placeholder="What would you like to say?"
              maxLength={144}
              disabled={isSending}
            />
            <div className="textarea-meta"><span>Enter for a new line · <kbd>Shift</kbd> + <kbd>Enter</kbd> to send</span><span>{message.length} / 144</span></div>
            <div className="emoji-toolbar">
              <button
                className="emoji-picker-toggle"
                type="button"
                aria-expanded={isEmojiPickerOpen}
                aria-controls="chatbox-emoji-picker"
                onClick={() => setIsEmojiPickerOpen((open) => !open)}
                disabled={isSending}
              >
                <span aria-hidden="true">😊</span> {isEmojiPickerOpen ? "Close emoji picker" : "Add emoji"}
              </button>
              <span>Pick an emoji to add it to your message</span>
            </div>
            {isEmojiPickerOpen && (
              <div className="emoji-picker" id="chatbox-emoji-picker" aria-label="Emoji picker">
                <div className="emoji-picker-categories" aria-label="Emoji categories">
                  {emojiCategories.map((category) => (
                    <button
                      key={category.label}
                      className={`emoji-category ${emojiCategory === category.label ? "active" : ""}`}
                      type="button"
                      aria-pressed={emojiCategory === category.label}
                      onClick={() => setEmojiCategory(category.label)}
                      disabled={isSending}
                    >
                      {category.label}
                    </button>
                  ))}
                </div>
                <div className="emoji-grid" aria-label={`${activeEmojiCategory.label} emojis`}>
                  {activeEmojiCategory.emojis.map((emoji, index) => (
                    <button
                      key={`${emoji}-${index}`}
                      className="emoji-option"
                      type="button"
                      aria-label={`Insert ${emoji} emoji`}
                      title={`Insert ${emoji}`}
                      onClick={() => insertEmoji(emoji)}
                      disabled={isSending}
                    >
                      {emoji}
                    </button>
                  ))}
                </div>
              </div>
            )}
            <div className="preview-box"><span className="preview-avatar">Y</span><span className="preview-message">{message || "Your message will appear here…"}</span><span className="preview-stars">✦</span></div>
            <div className="composer-actions">
              <span className="backend-hint"><span className="status-dot" /> {isDesktop ? "Send to your VRChat chatbox" : "Sending available in the desktop app"}</span>
              <div className="composer-action-buttons">
                <button className="secondary-button" type="button" onClick={() => {
                  setTemplateNameDraft(message.trim().split(/\r?\n/, 1)[0].slice(0, 40));
                  setIsSaveTemplateFormOpen((open) => !open);
                }} disabled={!message.trim() || isTemplateSaving || !isDesktop}>
                  {isTemplateSaving ? "Saving…" : "Save as template"}
                </button>
                <button className="primary-button" type="submit" disabled={!message.trim() || isSending}>
                  {isSending ? "Sending…" : "Send message"} <Icon name="send" size={16} />
                </button>
              </div>
            </div>
          </form>
          {isSaveTemplateFormOpen && (
            <div className="template-save-form">
              <label htmlFor="template-name">TEMPLATE NAME</label>
              <input id="template-name" maxLength={40} value={templateNameDraft} onChange={(event) => setTemplateNameDraft(event.currentTarget.value)} autoFocus />
              <div>
                <button type="button" className="template-cancel" onClick={() => setIsSaveTemplateFormOpen(false)}>Cancel</button>
                <button type="button" onClick={() => {
                  onSaveTemplate(templateNameDraft, message);
                  setIsSaveTemplateFormOpen(false);
                }} disabled={!templateNameDraft.trim() || isTemplateSaving}>Save template</button>
              </div>
            </div>
          )}
        </section>
        <aside className="chat-side">
          <section className="content-card history-card">
            <div className="history-heading">
              <span className="section-icon history-icon"><Icon name="clock" size={18} /></span>
              <span><strong>Reusable messages</strong><small>Templates and recent sends</small></span>
              <div className="history-tabs" role="tablist" aria-label="Reusable message lists">
                <button type="button" role="tab" aria-selected={listTab === "templates"} className={listTab === "templates" ? "active" : ""} onClick={() => setListTab("templates")}>Templates</button>
                <button type="button" role="tab" aria-selected={listTab === "history"} className={listTab === "history" ? "active" : ""} onClick={() => setListTab("history")}>Recent</button>
              </div>
            </div>
            {repeatStatus.active && (
              <div className="repeat-active" role="status">
                <span className="repeat-indicator" />
                <span className="repeat-active-copy">
                  <strong>Repeating {repeatStatus.messages?.length ?? 0} {repeatStatus.messages?.length === 1 ? "message" : "messages"}</strong>
                  <small>{repeatStatus.messages?.join(" → ")}</small>
                </span>
                <button type="button" className="repeat-stop-button" onClick={onStopRepeat}>Stop</button>
              </div>
            )}
            {listTab === "templates" && isDesktop && !templatesError && !isTemplatesLoading && templates.length > 0 && (
              <div className="repeat-selection-toolbar">
                <span>{selectedRepeatMessages.length} selected <small>· Repeats in order</small></span>
                <button
                  type="button"
                  onClick={() => onStartRepeat(selectedRepeatMessages)}
                  disabled={repeatStatus.active || selectedRepeatMessages.length === 0 || isTemplateSaving}
                >
                  Repeat selected
                </button>
              </div>
            )}
            {listTab === "templates" ? (
              !isDesktop ? (
                <div className="history-empty">Templates are available in the desktop app.</div>
              ) : templatesError ? (
                <div className="history-empty history-error" role="status">{templatesError}</div>
              ) : isTemplatesLoading ? (
                <div className="history-empty" role="status">Loading templates…</div>
              ) : templates.length === 0 ? (
                <div className="history-empty"><span className="history-empty-icon"><Icon name="message" size={18} /></span><strong>No templates yet</strong><span>Save a draft to reuse or repeat it later.</span></div>
              ) : (
                <div className="history-list" aria-label="Chatbox templates">
                  {templates.map((template, index) => (
                    <div className={`template-item ${selectedRepeatIndices.has(index) ? "selected" : ""}`} key={`${template.content}-${index}`}>
                      <div className="template-item-heading">
                        <input
                          type="checkbox"
                          checked={selectedRepeatIndices.has(index)}
                          onChange={(event) => {
                            const isSelected = event.currentTarget.checked;
                            setSelectedRepeatIndices((current) => {
                              const updated = new Set(current);
                              if (isSelected) updated.add(index);
                              else updated.delete(index);
                              return updated;
                            });
                          }}
                          disabled={repeatStatus.active || isTemplateSaving}
                          aria-label={`Include ${template.name || template.content.slice(0, 40)} in repeating messages`}
                        />
                        <strong className="template-name">{template.name || template.content.slice(0, 40)}</strong>
                      </div>
                      <span className="history-item-content">{template.content}</span>
                      <div className="template-item-actions">
                        <button type="button" onClick={() => setMessage(template.content.slice(0, 144))} disabled={isSending || isTemplateSaving}>Use</button>
                        <button type="button" onClick={() => {
                          if (editIndex === index) {
                            setEditIndex(null);
                            return;
                          }
                          setEditIndex(index);
                          setEditNameDraft(template.name || template.content.slice(0, 40));
                          setEditContentDraft(template.content);
                        }} disabled={isTemplateSaving || repeatStatus.active}>Edit</button>
                        <button type="button" className="template-delete" onClick={() => onDeleteTemplate(index)} disabled={isTemplateSaving}>Delete</button>
                      </div>
                      {editIndex === index && (
                        <form className="template-edit-form" onSubmit={async (event) => {
                          event.preventDefault();
                          if (await onUpdateTemplate(index, editNameDraft, editContentDraft)) {
                            setEditIndex(null);
                          }
                        }}>
                          <label htmlFor={`template-edit-name-${index}`}>TEMPLATE NAME</label>
                          <input
                            id={`template-edit-name-${index}`}
                            maxLength={40}
                            value={editNameDraft}
                            onChange={(event) => setEditNameDraft(event.currentTarget.value)}
                            autoFocus
                          />
                          <label htmlFor={`template-edit-content-${index}`}>MESSAGE <span>{editContentDraft.length} / 144</span></label>
                          <textarea
                            id={`template-edit-content-${index}`}
                            maxLength={144}
                            value={editContentDraft}
                            onChange={(event) => setEditContentDraft(event.currentTarget.value.slice(0, 144))}
                            rows={3}
                          />
                          <div className="template-edit-actions">
                            <button type="button" onClick={() => setEditIndex(null)} disabled={isTemplateSaving}>Cancel</button>
                            <button type="submit" disabled={!editNameDraft.trim() || !editContentDraft.trim() || editContentDraft.length > 144 || isTemplateSaving}>
                              {isTemplateSaving ? "Saving…" : "Save changes"}
                            </button>
                          </div>
                        </form>
                      )}
                    </div>
                  ))}
                </div>
              )
            ) : !isDesktop ? (
              <div className="history-empty">History is available in the desktop app.</div>
            ) : historyError ? (
              <div className="history-empty history-error" role="status">{historyError}</div>
            ) : isHistoryLoading ? (
              <div className="history-empty" role="status">Loading your messages…</div>
            ) : history.length === 0 ? (
              <div className="history-empty"><span className="history-empty-icon"><Icon name="message" size={18} /></span><strong>No messages yet</strong><span>Messages you send will show up here.</span></div>
            ) : (
              <div className="history-list" aria-label="Recent chatbox messages">
                {[...history].reverse().map((item, index) => (
                  <button
                    className="history-item"
                    key={`${item.sendTime}-${index}`}
                    type="button"
                    onClick={() => setMessage(item.originContent.slice(0, 144))}
                    disabled={isSending}
                    title="Use this message"
                  >
                    <span className="history-item-content">{item.originContent}</span>
                    <span className="history-item-meta">{new Date(item.sendTime * 1000).toLocaleString(undefined, { month: "short", day: "numeric", hour: "numeric", minute: "2-digit" })}<span>USE</span></span>
                  </button>
                ))}
              </div>
            )}
          </section>
        </aside>
      </div>
    </>
  );
}

function AvatarPage({ isDesktop }: { isDesktop: boolean }) {
  const [heightInput, setHeightInput] = useState("50");
  const [isApplying, setIsApplying] = useState(false);
  const [result, setResult] = useState<{ type: "success" | "error"; message: string } | null>(null);
  const [heightValues, setHeightValues] = useState<number[]>([]);
  const [isLoadingValues, setIsLoadingValues] = useState(isDesktop);
  const [isSavingValues, setIsSavingValues] = useState(false);
  const [valuesLoadError, setValuesLoadError] = useState("");
  const [valuesError, setValuesError] = useState("");

  useEffect(() => {
    if (!isDesktop) return;

    let cancelled = false;
    setIsLoadingValues(true);
    invoke<number[]>("get_avatar_height_values")
      .then((values) => {
        if (!cancelled) setHeightValues(values);
      })
      .catch((error: unknown) => {
        if (!cancelled) setValuesLoadError(`Could not load saved height shortcuts: ${String(error)}`);
      })
      .finally(() => {
        if (!cancelled) setIsLoadingValues(false);
      });

    return () => {
      cancelled = true;
    };
  }, [isDesktop]);

  async function applyHeight(value: number) {
    if (!isDesktop) {
      setResult({ type: "error", message: "Changing avatar height is only available in the VRC Toolbox desktop app." });
      return;
    }

    setIsApplying(true);
    setResult(null);
    try {
      await invoke("change_avatar_height", { height: value });
      setResult({ type: "success", message: `Avatar eye-height value set to ${value}.` });
    } catch (error) {
      setResult({ type: "error", message: `Could not change avatar height: ${String(error)}` });
    } finally {
      setIsApplying(false);
    }
  }

  function readHeightInput() {
    const height = Number(heightInput);
    if (!heightInput.trim() || !Number.isFinite(height) || height < 0.01 || height > 100) {
      setResult({ type: "error", message: "Enter a number from 0.01 to 100." });
      return null;
    }
    return height;
  }

  async function submitHeight(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const height = readHeightInput();
    if (height !== null) await applyHeight(height);
  }

  async function saveHeightValues(values: number[]) {
    if (isLoadingValues || isSavingValues || valuesLoadError) return;

    setIsSavingValues(true);
    setValuesError("");
    try {
      const savedValues = isDesktop
        ? await invoke<number[]>("save_avatar_height_values", { heightValues: values })
        : values;
      setHeightValues(savedValues);
    } catch (error) {
      setValuesError(`Could not save height shortcuts: ${String(error)}`);
    } finally {
      setIsSavingValues(false);
    }
  }

  async function addHeightValue() {
    const height = readHeightInput();
    if (height === null) return;
    if (heightValues.includes(height)) {
      setResult({ type: "error", message: "That height is already saved as a shortcut." });
      return;
    }
    await saveHeightValues([...heightValues, height]);
  }

  return (
    <>
      <PageIntro eyebrow="BUILT-IN TOOL · 02" title="Avatar Height Change" description="Adjust your avatar’s eye height using VRChat OSC." />
      <div className="settings-layout avatar-layout">
        <section className="content-card avatar-height-card">
          <div className="content-card-header">
            <span className="section-icon peach-icon"><Icon name="user" size={19} /></span>
            <span><strong>Avatar eye height</strong><small>Enter a value or use one of your saved shortcuts</small></span>
          </div>
          <form onSubmit={submitHeight}>
            <div className="avatar-height-label">
              <label className="field-label" htmlFor="avatar-height">HEIGHT VALUE</label>
            </div>
            <input
              className="avatar-height-number"
              id="avatar-height"
              type="number"
              min="0.01"
              max="100"
              step="0.01"
              value={heightInput}
              onChange={(event) => {
                setHeightInput(event.currentTarget.value);
                setResult(null);
              }}
              disabled={isApplying}
              aria-describedby="avatar-height-help"
            />
            <p className="field-help" id="avatar-height-help">Enter a value from 0.01 to 100. Applying sends it to your current avatar; it is not saved as a preference.</p>
            {result && <p className={`avatar-height-result ${result.type}`} role={result.type === "error" ? "alert" : "status"}>{result.message}</p>}
            <div className="settings-actions">
              <span className="backend-hint"><span className="status-dot" /> {isDesktop ? "Send via your local OSC connection" : "Available in the desktop app"}</span>
              <div className="avatar-height-actions">
                <button className="secondary-button avatar-save-button" type="button" onClick={addHeightValue} disabled={isLoadingValues || isSavingValues || Boolean(valuesLoadError)}>
                  {isSavingValues ? "Saving…" : "Save shortcut"}
                </button>
                <button className="primary-button" type="submit" disabled={isApplying}>{isApplying ? "Applying…" : "Apply height"} <Icon name="check" size={16} /></button>
              </div>
            </div>
          </form>
          <section className="avatar-shortcuts" aria-labelledby="avatar-shortcuts-heading">
            <div className="avatar-shortcuts-heading">
              <div><strong id="avatar-shortcuts-heading">Height shortcuts</strong><small>{isDesktop ? "Saved on this device" : "Preview only; shortcuts will not persist"}</small></div>
              {isSavingValues && <span role="status">Saving…</span>}
            </div>
            {valuesLoadError && <p className="avatar-height-result error" role="alert">{valuesLoadError}</p>}
            {valuesError && <p className="avatar-height-result error" role="alert">{valuesError}</p>}
            {isLoadingValues ? (
              <p className="avatar-shortcuts-empty" role="status">Loading saved shortcuts…</p>
            ) : valuesLoadError ? (
              <p className="avatar-shortcuts-empty">Saved shortcuts are unavailable until they can be loaded.</p>
            ) : heightValues.length === 0 ? (
              <p className="avatar-shortcuts-empty">No saved values yet. Enter a height and save it as a shortcut.</p>
            ) : (
              <div className="avatar-shortcuts-list" role="list" aria-label="Saved avatar height shortcuts">
                {heightValues.map((value) => (
                  <div className="avatar-shortcut" role="listitem" key={value}>
                    <button className="avatar-shortcut-apply" type="button" onClick={() => {
                      setHeightInput(String(value));
                      void applyHeight(value);
                    }} disabled={isApplying || isSavingValues || !isDesktop} aria-label={`Apply height ${value}`}>
                      {value}
                    </button>
                    <button className="avatar-shortcut-remove" type="button" onClick={() => void saveHeightValues(heightValues.filter((savedValue) => savedValue !== value))} disabled={isSavingValues || Boolean(valuesLoadError)} aria-label={`Remove height ${value} shortcut`} title={`Remove ${value}`}>
                      ×
                    </button>
                  </div>
                ))}
              </div>
            )}
          </section>
        </section>
      </div>
    </>
  );
}

function ComingSoonPage({ onBack }: { onBack: () => void }) {
  return (
    <>
      <PageIntro
        eyebrow="BUILT-IN TOOL · 03"
        title="Your controls, together."
        description="A considered home for supported VRChat input controls."
      />
      <section className="coming-card">
        <div className="coming-icon blue-icon"><Icon name="gamepad" size={25} /></div>
        <span className="coming-pill"><Icon name="clock" size={14} /> IN THE WORKS</span>
        <h2>This one needs extra care.</h2>
        <p>Input controls need reliable safety and release behavior. We’ll take the time to get that right before adding them.</p>
        <div className="safety-note"><Icon name="shield" size={17} /><span>Controls that can affect movement should always be safe to stop.</span></div>
        <button className="secondary-button" onClick={onBack}>Back to your tools <Icon name="arrow" size={16} /></button>
      </section>
    </>
  );
}

function SettingsPage({
  target,
  setTarget,
  onSubmit,
  savedTarget,
  connectionStatus,
}: {
  target: string;
  setTarget: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  savedTarget: string;
  connectionStatus: ConnectionStatus;
}) {
  return (
    <>
      <PageIntro eyebrow="MAKE IT YOURS" title="The little details." description="Your preferences for connecting VRC Toolbox to VRChat." />
      <div className="settings-layout">
        <section className="content-card settings-card">
          <div className="content-card-header"><span className="section-icon settings-icon"><Icon name="sliders" size={19} /></span><span><strong>OSC connection</strong><small>VRChat’s local OSC endpoint</small></span><span className={`mini-status status-label-${connectionStatus}`}><span className={`status-dot status-${connectionStatus}`} /> {connectionStatusLabel(connectionStatus).toUpperCase()}</span></div>
          <form onSubmit={onSubmit}>
            <label className="field-label" htmlFor="osc-target">TARGET ADDRESS</label>
            <div className="input-with-icon"><Icon name="wifi" size={17} /><input id="osc-target" value={target} onChange={(event) => setTarget(event.currentTarget.value)} placeholder="127.0.0.1:9000" /></div>
            <p className="field-help">VRChat’s default OSC address is 127.0.0.1:9000. You can change this if your setup uses a different target.</p>
            <div className="settings-saved"><span className="settings-saved-icon"><Icon name="check" size={15} /></span><span><strong>Saved target</strong><small>{savedTarget || "No target configured"}</small></span></div>
            <div className="settings-actions"><span className="backend-hint"><Icon name="clock" size={15} /> {isTauri() ? "Stored on this device" : "Preview only"}</span><button className="primary-button" type="submit" disabled={!target.trim()}>Save preference <Icon name="arrow" size={16} /></button></div>
          </form>
        </section>
        <aside className="settings-side-card"><span className="settings-side-icon"><Icon name="shield" size={19} /></span><span className="eyebrow">YOUR PRIVACY</span><h3>Local by design.</h3><p>VRC Toolbox talks to VRChat on your device. This early preview doesn’t connect to a cloud service.</p><div className="privacy-tag"><span /> DEVICE ONLY</div></aside>
      </div>
    </>
  );
}

function PageIntro({ eyebrow, title, description }: { eyebrow: string; title: string; description: string }) {
  return (
    <div className="page-intro">
      <div className="eyebrow"><span className="eyebrow-line" /> {eyebrow}</div>
      <h1>{title}</h1>
      <p>{description}</p>
    </div>
  );
}

export default App;
