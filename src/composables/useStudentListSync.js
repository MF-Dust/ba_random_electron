import { ref } from 'vue'
import { studentListToText } from '../studentListText'

export function useStudentListSync(appApi, config, addLog) {
  const rawListText = ref('')
  let textSyncTimer = null
  let textSyncRunId = 0
  let lastSyncedText = ''

  const syncTextToList = async ({ updateText = false } = {}) => {
    if (textSyncTimer) {
      window.clearTimeout(textSyncTimer)
      textSyncTimer = null
    }
    const rawText = rawListText.value
    if (!updateText && rawText === lastSyncedText) {
      return
    }
    const runId = ++textSyncRunId
    const result = await appApi.parseStudentListText(rawText, config.value.studentList || [])
    if (runId !== textSyncRunId) {
      return
    }

    config.value.studentList = result.studentList || []
    if (updateText) {
      rawListText.value = result.normalizedText || ''
      lastSyncedText = rawListText.value
    } else {
      lastSyncedText = rawText
    }
  }

  const scheduleTextSync = () => {
    if (textSyncTimer) {
      window.clearTimeout(textSyncTimer)
    }
    textSyncTimer = window.setTimeout(() => {
      syncTextToList().catch((error) => {
        console.error('同步名单失败:', error)
        addLog('error', '同步名单失败...老师检查一下输入内容～')
      })
    }, 300)
  }

  const syncListToText = () => {
    if (textSyncTimer) {
      window.clearTimeout(textSyncTimer)
      textSyncTimer = null
    }
    textSyncRunId += 1
    rawListText.value = studentListToText(config.value.studentList || [])
    lastSyncedText = rawListText.value
  }

  const removeStudent = (index) => {
    config.value.studentList.splice(index, 1)
    syncListToText()
  }

  const resetWeights = () => {
    config.value.studentList.forEach(s => { s.weight = 1.0 })
  }

  const handleFileImport = async () => {
    try {
      const result = await appApi.importStudentListFromFile(config.value.studentList || [])
      if (!result) return
      config.value.studentList = result.studentList
      rawListText.value = result.normalizedText
      lastSyncedText = rawListText.value
      addLog('info', `已经导入 ${result.studentList.length} 名学生啦～`)
    } catch (error) {
      console.error('导入名单失败:', error)
      addLog('error', '导入名单失败...老师检查一下文件内容～')
    }
  }

  const stopTextSync = () => {
    if (textSyncTimer) {
      window.clearTimeout(textSyncTimer)
      textSyncTimer = null
    }
  }

  return {
    rawListText,
    syncTextToList,
    scheduleTextSync,
    syncListToText,
    removeStudent,
    resetWeights,
    handleFileImport,
    stopTextSync
  }
}
