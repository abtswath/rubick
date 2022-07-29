<template>
    <NThing>
        <template #header>
            搜索历史
        </template>
        <template #header-extra>
            <NButton text @click="clear">
                <NIcon>
                    <DeleteForeverOutlined />
                </NIcon>
                清空
            </NButton>
        </template>
        <NSpace>
            <template v-for="item in histories">
                <NTag size="small" :bordered="false" @click="() => emit('select', item)">
                    {{ item }}
                </NTag>
            </template>
        </NSpace>
    </NThing>
</template>

<script lang="ts" setup>
import { NThing, NIcon, NButton, NSpace, NTag } from 'naive-ui';
import { DeleteForeverOutlined } from '@vicons/material';
import { useStore } from '@/store';
import { computed } from 'vue';

const store = useStore();
const histories = computed(() => store.state.searchHistory.histories);
const clear = () => store.dispatch('searchHistory/clear');

const emit = defineEmits<{
    (e: 'select', item: string): void
}>();
</script>

<style lang="scss" scoped>
</style>
