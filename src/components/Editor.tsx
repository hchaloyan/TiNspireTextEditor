import "./Editor.css";

interface Props {
  content: string;
  onChange: (v: string) => void;
  split: boolean;
}

export default function Editor({ content, onChange, split }: Props) {
  return (
    <div className={`editor-pane ${split ? "split" : "full"}`}>
      <div className="pane-label">markdown</div>
      <textarea
        className="editor-textarea"
        value={content}
        onChange={(e) => onChange(e.target.value)}
        spellCheck={false}
        autoComplete="off"
        autoCorrect="off"
        autoCapitalize="off"
      />
    </div>
  );
}
