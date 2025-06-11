/**
 * 下载相关的 Tauri API 调用
 */

import { invoke } from '@tauri-apps/api/core'
import type { DownloadResponse } from './types'

/**
 * 下载GitHub仓库到plugins目录
 * @returns Promise<DownloadResponse> 下载结果
 */
export async function downloadGithubRepo(): Promise<DownloadResponse> {
  try {
    const response = await invoke<DownloadResponse>('download_github_repo')
    return response
  } catch (error) {
    console.error('Failed to download GitHub repo:', error)
    throw error
  }
}
