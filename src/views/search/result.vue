<template>
    <div class="result">
        <NScrollbar id="search-result" @scroll="handleScroll">
            <VirtualList :data="resources" />
        </NScrollbar>
    </div>
</template>

<script lang="ts" setup>
import { NScrollbar } from 'naive-ui';
import { SearchResult } from '@/compositions/use-search-resources';
import { VNode, h } from 'vue';
import useVirtualList from '@/compositions/use-virtual-list';
import Item from './item.vue';

defineProps<{
    resources: SearchResult[]
}>();

const { VirtualList, handleScroll } = useVirtualList<SearchResult>({
    el: "#search-result",
    renderer: (resource): VNode => {
        return h(Item, {
            resource
        });
    }
});
</script>

<style lang="scss" scoped>
.result {
    height: calc(100vh - 140px);
}
</style>
