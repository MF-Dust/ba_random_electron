import { ref } from 'vue'

export const tabGroups = [
  {
    title: '名单管理',
    tabs: [
      {
        key: 'list',
        label: '导入名单',
        title: '导入名单',
        hint: '老师～把名单交给我就好啦！粘贴文字或者导入文件都可以哦',
        icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>'
      },
      {
        key: 'students',
        label: '查看 & 调整',
        title: '名单一览',
        hint: '这里可以看到所有人的名字和权重哦，想调整的话尽管来～',
        icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>'
      }
    ]
  },
  {
    title: '界面定制',
    tabs: [
      {
        key: 'floating',
        label: '悬浮按钮',
        title: '悬浮按钮',
        hint: '悬浮按钮的大小、透明度、位置……老师想怎么摆都行！',
        icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="12" cy="12" r="3"/></svg>'
      },
      {
        key: 'pickCount',
        label: '抽选演出',
        title: '抽选演出',
        hint: '音效和动画的设置都在这里～让抽选的瞬间更有仪式感吧！',
        icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>'
      }
    ]
  },
  {
    title: '进阶设置',
    tabs: [
      {
        key: 'web',
        label: '系统 & 更新',
        title: '系统 & 更新',
        hint: '这里是比较进阶的设置了，一般保持默认就好……不过老师想改的话我也不拦着啦～',
        icon: '<svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>'
      }
    ]
  }
]

const tabs = tabGroups.flatMap(group => group.tabs)

export function useConfigTabs() {
  const activeTab = ref('list')

  const switchTab = (tab) => {
    activeTab.value = tab
  }

  const getActiveTabMeta = () => tabs.find(tab => tab.key === activeTab.value)

  return {
    tabGroups,
    tabs,
    activeTab,
    switchTab,
    getActiveTabMeta
  }
}
