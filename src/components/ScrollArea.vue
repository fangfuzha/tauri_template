<script setup lang="ts">
import type { PartialOptions, EventListeners } from "overlayscrollbars";
import type { OverlayScrollbarsComponentProps } from "overlayscrollbars-vue";
import { computed, useAttrs, type HTMLAttributes } from "vue";
import { OverlayScrollbarsComponent } from "overlayscrollbars-vue";

defineOptions({
  inheritAttrs: false,
});

type ScrollAreaAxis = "vertical" | "horizontal" | "both";

interface ScrollAreaProps {
  axis?: ScrollAreaAxis;
  element?: OverlayScrollbarsComponentProps["element"];
  options?: PartialOptions;
  events?: EventListeners;
  defer?: OverlayScrollbarsComponentProps["defer"];
  contentClass?: HTMLAttributes["class"];
  fillContent?: boolean;
}

const props = withDefaults(defineProps<ScrollAreaProps>(), {
  axis: "vertical",
  element: "div",
  options: undefined,
  events: undefined,
  defer: true,
  contentClass: undefined,
  fillContent: true,
});

const attrs = useAttrs();

const defaultOptions = computed<PartialOptions>(() => {
  const overflow: NonNullable<PartialOptions["overflow"]> =
    props.axis === "horizontal"
      ? { x: "scroll", y: "hidden" }
      : props.axis === "both"
        ? { x: "scroll", y: "scroll" }
        : { x: "hidden", y: "scroll" };

  return {
    overflow,
    scrollbars: {
      theme: "os-theme-app",
      autoHide: "move",
      autoHideDelay: 450,
    },
  };
});

const mergedOptions = computed<PartialOptions>(() => ({
  ...defaultOptions.value,
  ...props.options,
  overflow: {
    ...defaultOptions.value.overflow,
    ...props.options?.overflow,
  },
  scrollbars: {
    ...defaultOptions.value.scrollbars,
    ...props.options?.scrollbars,
  },
  update: {
    ...defaultOptions.value.update,
    ...props.options?.update,
  },
}));
</script>

<template>
  <OverlayScrollbarsComponent
    v-bind="attrs"
    class="ui-scroll-area"
    :class="`ui-scroll-area--${axis}`"
    :element="element"
    :options="mergedOptions"
    :events="events"
    :defer="defer"
  >
    <div
      class="ui-scroll-area__content"
      :class="[
        contentClass,
        {
          'ui-scroll-area__content--fill-x': fillContent && axis !== 'vertical',
          'ui-scroll-area__content--fill-y':
            fillContent && axis !== 'horizontal',
        },
      ]"
    >
      <slot />
    </div>
  </OverlayScrollbarsComponent>
</template>

<style scoped>
.ui-scroll-area {
  width: 100%;
  height: 100%;
  min-width: 0;
  min-height: 0;
  max-width: 100%;
  max-height: 100%;
  overflow: hidden;
}

.ui-scroll-area--horizontal {
  height: auto;
}

.ui-scroll-area__content {
  box-sizing: border-box;
}

.ui-scroll-area__content--fill-x {
  width: 100%;
  min-width: 100%;
}

.ui-scroll-area__content--fill-y {
  height: 100%;
  min-height: 100%;
}
</style>
