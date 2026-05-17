import { ref } from 'vue'
import { DEFAULT_ADMIN_TASK_NAME, createDefaultConfig } from '../configDefaults'
import { studentListToText } from '../studentListText'

export function useAppConfig(appApi, addLog) {
  const config = ref(createDefaultConfig())
  const isDebugMode = ref(false)
  const isAdmin = ref(false)
  const appVersion = ref('0.0.0')
  const defaultExePath = ref('')

  const applyDefaultAutoStartPath = () => {
    if (!config.value || !config.value.webConfig) return
    if (!config.value.webConfig.adminAutoStartPath && defaultExePath.value) {
      config.value.webConfig.adminAutoStartPath = defaultExePath.value
    }
  }

  const fetchConfig = async (rawListText) => {
    try {
      config.value = await appApi.getConfig()
      if (rawListText) {
        rawListText.value = studentListToText(config.value.studentList || [])
      }
      applyDefaultAutoStartPath()
      addLog('info', '配置加载完成～')
    } catch (error) {
      console.error('加载配置失败:', error)
      addLog('error', '加载配置失败...老师检查一下后端有没有启动～')
      window.alert('配置页面初始化失败啦...')
    }
  }

  const fetchAppInfo = async () => {
    try {
      const response = await appApi.getAppInfo()
      isDebugMode.value = Boolean(response && response.isDebugMode)
      isAdmin.value = Boolean(response && response.isAdmin)
      appVersion.value = response && response.version ? response.version : '0.0.0'
      defaultExePath.value = response && response.exePath ? response.exePath : ''
      applyDefaultAutoStartPath()
    } catch (_error) {
      isDebugMode.value = false
      isAdmin.value = false
      appVersion.value = '0.0.0'
      defaultExePath.value = ''
    }
  }

  const saveConfig = async (syncTextToList) => {
    try {
      await syncTextToList({ updateText: true })
      await appApi.saveConfig(config.value)
      addLog('success', '配置保存成功！已经生效啦～')
      window.alert('配置保存成功！已经生效啦～')
    } catch (error) {
      console.error('保存配置失败:', error)
      addLog('error', '保存失败...老师检查一下输入内容～')
      window.alert('保存失败...老师检查一下输入内容～')
    }
  }

  const requestAdminElevation = async () => {
    try {
      const response = await appApi.requestAdminElevation()
      addLog(response.ok ? 'info' : 'error', response.message || '已发送管理员权限请求！')
      window.alert(response.message || '已发送管理员权限请求！')
    } catch (error) {
      console.error('申请管理员权限失败:', error)
      const message = error?.message || '申请管理员权限失败啦...'
      addLog('error', message)
      window.alert(`${message}，请查看日志。`)
    }
  }

  const createAdminStartupTask = async () => {
    try {
      const fallbackPath = defaultExePath.value || ''
      const payload = {
        exePath: String(config.value.webConfig.adminAutoStartPath || fallbackPath).trim(),
        taskName: String(config.value.webConfig.adminAutoStartTaskName || DEFAULT_ADMIN_TASK_NAME).trim()
      }
      if (!payload.exePath) {
        window.alert('老师先填一下可执行文件的路径哦～')
        return
      }
      const response = await appApi.createAdminStartupTask(payload.exePath, payload.taskName)
      addLog(response.ok ? 'success' : 'error', response.message || '开机任务已经创建/更新啦～')
      window.alert(response.message || '开机任务已经创建/更新啦～')
    } catch (error) {
      console.error('创建计划任务失败:', error)
      addLog('error', '创建开机任务失败啦...')
      window.alert('创建开机任务失败...老师看看日志吧～')
    }
  }

  return {
    config,
    isDebugMode,
    isAdmin,
    appVersion,
    defaultExePath,
    fetchConfig,
    fetchAppInfo,
    saveConfig,
    requestAdminElevation,
    createAdminStartupTask
  }
}
