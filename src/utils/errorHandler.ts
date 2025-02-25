import type { MessageApi } from 'naive-ui'

// 错误代码定义（与后端一致）
export enum ErrorCode {
  Network = 1000,
  Parse = 2000,
  IO = 3000,
  Download = 4000,
  Config = 5000,
  Json = 6000,
  Zip = 7000,
  Validation = 8000,
  NotFound = 9000,
  Permission = 10000,
  Execution = 11000,
  Unknown = 99999,
}

// 错误响应类型
export interface ErrorResponse {
  code: ErrorCode
  message: string
  details: string
}

// 通用错误对象接口
interface ErrorObject {
  code?: number
  message?: string
  details?: string
  [key: string]: unknown
}

// 错误严重程度
export enum ErrorSeverity {
  Info, // 信息提示，不影响使用
  Warning, // 警告，可能有一些功能受限
  Error, // 错误，功能无法正常使用
  Critical, // 严重错误，应用可能无法继续运行
}

// 获取错误严重程度
export function getErrorSeverity(code: ErrorCode): ErrorSeverity {
  // 根据错误代码确定严重性
  switch (code) {
    case ErrorCode.Network:
    case ErrorCode.Download:
      return ErrorSeverity.Warning

    case ErrorCode.IO:
    case ErrorCode.Zip:
    case ErrorCode.Json:
    case ErrorCode.Parse:
      return ErrorSeverity.Error

    case ErrorCode.Permission:
    case ErrorCode.Execution:
      return ErrorSeverity.Critical

    default:
      return ErrorSeverity.Error
  }
}

// 默认错误消息
export function getDefaultErrorMessage(code: ErrorCode): string {
  switch (code) {
    case ErrorCode.Network:
      return '网络连接失败，请检查您的网络设置'
    case ErrorCode.Parse:
      return '数据解析失败，请稍后重试'
    case ErrorCode.IO:
      return '文件读写错误，请检查磁盘空间和权限'
    case ErrorCode.Download:
      return '下载失败，请稍后重试'
    case ErrorCode.NotFound:
      return '找不到请求的资源'
    case ErrorCode.Permission:
      return '权限不足，请以管理员身份运行程序'
    default:
      return '发生未知错误，请尝试重启应用'
  }
}

// 统一错误处理函数
export function handleError(
  error: unknown,
  messageApi: MessageApi | undefined,
  logError: boolean = true,
): void {
  let errorMessage = '未知错误'
  let errorCode = ErrorCode.Unknown
  let errorDetails = ''

  // 解析错误对象
  if (typeof error === 'string') {
    errorMessage = error
  } else if (error instanceof Error) {
    errorMessage = error.message
    errorDetails = error.stack || ''
  } else if (typeof error === 'object' && error !== null) {
    // 尝试解析Tauri API返回的错误
    const errorObj = error as ErrorObject
    if (errorObj.code && errorObj.message) {
      errorCode = errorObj.code as ErrorCode
      errorMessage = errorObj.message as string
      errorDetails = errorObj.details || ''
    }
  }

  // 日志记录
  if (logError) {
    console.error('错误详情:', {
      code: errorCode,
      message: errorMessage,
      details: errorDetails,
    })
  }

  // 显示错误消息
  if (messageApi) {
    const severity = getErrorSeverity(errorCode)

    // 根据错误严重程度选择不同提示类型
    switch (severity) {
      case ErrorSeverity.Info:
        messageApi.info(errorMessage)
        break
      case ErrorSeverity.Warning:
        messageApi.warning(errorMessage)
        break
      case ErrorSeverity.Error:
      case ErrorSeverity.Critical:
        messageApi.error(errorMessage, { duration: 5000 })
        break
    }
  }
}
