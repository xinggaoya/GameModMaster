declare module '@tauri-apps/api/shell' {
  export function open(path: string): Promise<void>
}

declare module '@tauri-apps/api/fs' {
  export function readTextFile(path: string): Promise<string>
  export function writeTextFile(path: string, contents: string): Promise<void>
  export function readBinaryFile(path: string): Promise<Uint8Array>
  export function writeBinaryFile(path: string, contents: Uint8Array): Promise<void>
  export function createDir(path: string, recursive?: boolean): Promise<void>
  export function removeDir(path: string, recursive?: boolean): Promise<void>
  export function exists(path: string): Promise<boolean>
  export function readDir(path: string): Promise<FileEntry[]>
  export function removeFile(path: string): Promise<void>
  export function renameFile(oldPath: string, newPath: string): Promise<void>

  export interface FileEntry {
    path: string
    name: string
    children?: FileEntry[]
  }
}
