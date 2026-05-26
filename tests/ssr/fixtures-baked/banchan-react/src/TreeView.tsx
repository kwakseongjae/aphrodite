"use client";

import { useState, ReactNode } from "react";
import { cn } from "./cn";

export interface TreeNodeData {
  id: string;
  label: ReactNode;
  children?: TreeNodeData[];
}

export interface TreeViewProps {
  nodes: TreeNodeData[];
  selectedId?: string;
  onSelect?: (id: string) => void;
  defaultExpanded?: string[];
  className?: string;
}

export function TreeView({ nodes, selectedId, onSelect, defaultExpanded = [], className }: TreeViewProps) {
  const [expanded, setExpanded] = useState<Set<string>>(new Set(defaultExpanded));
  const toggle = (id: string) => {
    setExpanded((s) => {
      const next = new Set(s);
      next.has(id) ? next.delete(id) : next.add(id);
      return next;
    });
  };
  return (
    <ul role="tree" className={cn("aph-tree", className)}>
      {nodes.map((n) => <TreeNode key={n.id} node={n} expanded={expanded} toggle={toggle} selectedId={selectedId} onSelect={onSelect} />)}
    </ul>
  );
}

export function TreeNode({ node, expanded, toggle, selectedId, onSelect }: { node: TreeNodeData; expanded: Set<string>; toggle: (id: string) => void; selectedId?: string; onSelect?: (id: string) => void }) {
  const isOpen = expanded.has(node.id);
  const isLeaf = !node.children || node.children.length === 0;
  return (
    <li role="treeitem" aria-expanded={isLeaf ? undefined : isOpen}>
      <div
        role="presentation"
        className="aph-tree__node"
        aria-selected={selectedId === node.id}
        onClick={() => { if (!isLeaf) toggle(node.id); onSelect?.(node.id); }}
      >
        <span className={cn("aph-tree__caret", isLeaf && "aph-tree__caret--leaf")} aria-hidden="true">{isOpen ? "▾" : "▸"}</span>
        <span>{node.label}</span>
      </div>
      {!isLeaf && isOpen && (
        <ul role="group" className="aph-tree__children">
          {node.children!.map((c) => <TreeNode key={c.id} node={c} expanded={expanded} toggle={toggle} selectedId={selectedId} onSelect={onSelect} />)}
        </ul>
      )}
    </li>
  );
}
