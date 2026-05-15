import { ref } from 'vue'

const tabs = ['list', 'students', 'floating', 'pickCount', 'web']

export function useConfigTabs() {
  const activeTab = ref('list')

  const switchTab = (tab) => {
    activeTab.value = tab
  }

  return {
    tabs,
    activeTab,
    switchTab
  }
}
