export interface Trainer {
  id: string
  name: string
  version: string
  game_version: string
  download_url: string
  description: string
  thumbnail: string
  download_count: number
  last_update: string
  // 本地安装相关信息
  installed_path?: string
  install_time?: string
  last_launch_time?: string | null
}

export interface InstalledTrainer extends Trainer {
  installed_path: string
  install_time: string
  last_launch_time: string | null
}
