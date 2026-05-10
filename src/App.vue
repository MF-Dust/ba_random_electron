<template>
  <router-view />
</template>

<script setup>
import { useRoute } from 'vue-router'
import { watch } from 'vue'

const route = useRoute()
watch(
  () => route.path,
  (newPath) => {
    if (newPath === '/config') {
      document.body.classList.add('is-config-page')
      document.documentElement.classList.add('is-config-page')
    } else {
      document.body.classList.remove('is-config-page')
      document.documentElement.classList.remove('is-config-page')
    }
  },
  { immediate: true }
)
</script>

<style>
* {
  box-sizing: border-box;
}

html,
body,
#app {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  font-family: "Microsoft YaHei UI", "PingFang SC", sans-serif;
}

html:not(.is-config-page),
body:not(.is-config-page),
html:not(.is-config-page) > body > #app {
  background: transparent;
  overflow: hidden;
  user-select: none;
}

html.is-config-page,
body.is-config-page,
html.is-config-page > body > #app {
  background: #f4f5f7;
  overflow: auto;
  user-select: text;
}
</style>
