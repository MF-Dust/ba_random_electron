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
        <span class="ba-nav-icon" v-html="tab.icon"></span>
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
