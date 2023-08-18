<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { MessageProps } from "./type";

withDefaults(defineProps<MessageProps>(), {
  type: "success",
  message: "",
});

const visible = ref(false);

onMounted(() => {
  visible.value = true;
});

const style = reactive({
  success: {
    backgroundColor: "#f0f9eb",
    color: "#67c23a",
  },
  error: {
    backgroundColor: "#fef0f0",
    color: "#f56c6c",
  },
  warning: {
    backgroundColor: "#fdf6ec",
    color: "#e6a23c",
  },
  info: {
    backgroundColor: "#edf2fc",
    color: "#409eff",
  },
});

const icon = reactive({
  success: "mingcute:check-circle-fill",
  error: "mingcute:close-circle-fill",
  warning: "mingcute:alert-fill",
  info: "mingcute:alert-octagon-fill",
});
</script>

<template>
  <Transition name="down">
    <div class="j-message" :style="style[type]" v-show="visible">
      <Icon :icon="icon[type]" />
      <span class="text">{{ message }}</span>
    </div>
  </Transition>
</template>

<style lang="less">
.down {
  &-enter {
    &-from {
      transform: translate3d(0, -75px, 0);
      opacity: 0;
    }
    &-active {
      transition: all 0.5s;
    }
    &-to {
      transform: none;
      opacity: 1;
    }
  }
}
.j-message {
  width: fit-content;
  position: fixed;
  left: 0;
  right: 0;
  margin: auto;
  z-index: 9999;
  top: 3rem;
  line-height: 3rem;
  padding: 0 1.5rem;
  border-radius: 0.4rem;
  display: flex;
  align-items: center;
  span {
    margin-left: 0.5rem;
  }
}
</style>
