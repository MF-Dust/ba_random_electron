const AppRoot = {
  name: 'AppRoot',
  components: {
    FloatingButton
  },
  setup() {
    const { ref, onMounted } = Vue;

    const sizePx = ref(50);
    const transparencyPercent = ref(20);

    async function initConfig() {
      if (!window.floatingButtonApi) return;
      const cfg = await window.floatingButtonApi.getConfig();
      sizePx.value = Math.round(50 * (cfg.sizePercent / 100));
      transparencyPercent.value = cfg.transparencyPercent;
    }

    function handleFloatingButtonClick() {
      if (window.floatingButtonApi) {
        window.floatingButtonApi.onClick();
      }
    }

    onMounted(() => {
      initConfig();
    });

    return {
      sizePx,
      transparencyPercent,
      handleFloatingButtonClick
    };
  },
  template: `
    <FloatingButton
      :size-px="sizePx"
      :transparency-percent="transparencyPercent"
      @click="handleFloatingButtonClick"
    />
  `
};
