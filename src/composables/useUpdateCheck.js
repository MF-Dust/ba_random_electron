import { ref } from 'vue'

const releasePageUrl = 'https://github.com/MF-Dust/KVRandom/releases/latest'

export function useUpdateCheck(appApi, addLog) {
  const updateState = ref({
    loading: false,
    status: 'idle',
    title: '还没检查过更新呢～',
    detail: '',
    commitUrl: '',
    releaseUrl: ''
  })

  const checkUpdate = async () => {
    addLog('info', '开始检查更新！')
    updateState.value = {
      loading: true,
      status: 'loading',
      title: '正在检查更新...',
      detail: '',
      commitUrl: '',
      releaseUrl: ''
    }

    try {
      const result = await appApi.checkUpdate()
      if (result && Array.isArray(result.debug)) {
        result.debug.forEach((line) => addLog('info', `更新调试: ${line}`))
      }

      updateState.value = {
        loading: false,
        status: result.status || 'error',
        title: result.title || '检查更新失败啦...',
        detail: result.detail || '老师检查一下网络，或者等会儿再试～',
        commitUrl: result.commitUrl || '',
        releaseUrl: result.releaseUrl || releasePageUrl
      }
      if (result.status === 'update') {
        addLog('success', '发现新版本啦！')
      } else if (result.status === 'ok') {
        addLog('success', '已经是最新版啦～')
      } else if (result.status === 'easter') {
        addLog('info', '本地版本比远程还新呢！')
      } else {
        addLog('error', result.detail || '检查更新失败')
      }
    } catch (error) {
      console.error('检查更新失败:', error)
      const message = error && error.message ? String(error.message) : ''
      addLog('error', `检查更新失败${message ? `: ${message}` : ''}`)
      updateState.value = {
        loading: false,
        status: 'error',
        title: '检查更新失败啦...',
        detail: '老师检查一下网络，或者等会儿再试～',
        commitUrl: '',
        releaseUrl: releasePageUrl
      }
    }
  }

  return {
    updateState,
    checkUpdate
  }
}
