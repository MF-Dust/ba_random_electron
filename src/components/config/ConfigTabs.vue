<template>
  <nav class="ba-sidebar-nav">
    <div class="ba-nav-group" v-for="group in tabGroups" :key="group.title">
      <div class="ba-nav-group-title">{{ group.title }}</div>
      <button
        v-for="tab in group.tabs"
        :key="tab.key"
        type="button"
        class="ba-nav-item"
        :class="{ active: activeTab === tab.key }"
        @click="$emit('switch-tab', tab.key)"
      >
        <span class="ba-nav-icon">
          <svg v-if="tab.key === 'list'" viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14 2 14 8 20 8"/>
            <line x1="16" y1="13" x2="8" y2="13"/>
            <line x1="16" y1="17" x2="8" y2="17"/>
            <polyline points="10 9 9 9 8 9"/>
          </svg>
          <svg v-else-if="tab.key === 'students'" viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
            <circle cx="9" cy="7" r="4"/>
            <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
            <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
          </svg>
          <svg v-else-if="tab.key === 'floating'" viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
            <circle cx="12" cy="12" r="3"/>
          </svg>
          <svg v-else-if="tab.key === 'pickCount'" viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
          </svg>
          <svg v-else viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
        </span>
        <span class="ba-nav-label">{{ tab.label }}</span>
      </button>
    </div>
  </nav>
</template>

<script setup>
defineProps({
  activeTab: {
    type: String,
    required: true
  },
  tabGroups: {
    type: Array,
    required: true
  }
})

defineEmits(['switch-tab'])
</script>

<style scoped>
.ba-sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 4px 0;
}

.ba-nav-group {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.ba-nav-group + .ba-nav-group {
  margin-top: 6px;
  padding-top: 10px;
  border-top: 1px solid rgba(18, 138, 250, 0.08);
}

.ba-nav-group-title {
  font-size: 11px;
  font-weight: 700;
  color: #8ca3bf;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  padding: 4px 16px 4px;
  user-select: none;
}

.ba-nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border: none;
  border-radius: 10px;
  background: transparent;
  color: #5a7394;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.22s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  text-align: left;
  white-space: nowrap;
  font-family: inherit;
}

.ba-nav-item::before {
  content: "";
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%) scaleY(0);
  width: 4px;
  height: 24px;
  border-radius: 0 4px 4px 0;
  background: linear-gradient(180deg, #3ea8ff 0%, #128afa 100%);
  transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.ba-nav-item:hover {
  background: rgba(18, 138, 250, 0.06);
  color: #2878c8;
}

.ba-nav-item.active {
  background: linear-gradient(135deg, rgba(18, 138, 250, 0.10) 0%, rgba(18, 138, 250, 0.05) 100%);
  color: #128afa;
}

.ba-nav-item.active::before {
  transform: translateY(-50%) scaleY(1);
}

.ba-nav-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

/* Responsive: hide group titles on narrow sidebar */
@media (max-width: 768px) {
  .ba-nav-group-title {
    display: none;
  }
}
</style>
