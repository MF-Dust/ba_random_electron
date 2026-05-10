import { ref } from 'vue'

const tabs = ['list', 'students', 'floating', 'pickCount', 'web']

export function useConfigTabs() {
  const activeTab = ref('list')
  const transitionName = ref('slide-left')

  const switchTab = (tab) => {
    const currentIndex = tabs.indexOf(activeTab.value)
    const nextIndex = tabs.indexOf(tab)
    transitionName.value = nextIndex > currentIndex ? 'slide-left' : 'slide-right'
    activeTab.value = tab
  }

  return {
    tabs,
    activeTab,
    transitionName,
    switchTab
  }
}
