import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { FileUploader } from "./FileUploader";

const meta: Meta<typeof FileUploader> = { component: FileUploader };
export default meta;
type Story = StoryObj<typeof FileUploader>;

export const Default: Story = {
  render: () => {
    const [files, setFiles] = useState<string[]>([]);
    return (
      <div style={{ width: 360 }}>
        <FileUploader multiple accept="image/*" onFiles={(fs) => setFiles(fs.map((f) => f.name))} />
        {files.length > 0 && <ul>{files.map((f) => <li key={f}>{f}</li>)}</ul>}
      </div>
    );
  },
};
