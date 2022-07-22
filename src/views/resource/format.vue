<template>
    <NTabs ref="tabRef" type="line" animated v-model:value="activated">
        <template v-for="format in data">
            <NTabPane :name="format.id" :tab="format.format">
                <Series :data="series" />
            </NTabPane>
        </template>
    </NTabs>
</template>

<script lang="ts" setup>
import { NTabs, NTabPane } from 'naive-ui';
import { Format } from '@/compositions/use-resource';
import { nextTick, computed, ref, watchEffect } from 'vue';
import Series from './series.vue';

const props = defineProps<{
    data: Format[]
}>();

const activated = ref(0);
const tabRef = ref<typeof NTabs>();

watchEffect(() => {
    if (props.data.length > 0) {
        activated.value = props.data[0].id;
        nextTick(() => {
            tabRef.value?.syncBarPosition();
        });
    }
});

const series = computed(() => {
    const result = props.data.filter(item => item.id === activated.value);
    return result.length > 0 ? result[0].series : [];
});

</script>

<style lang="scss" scoped>
</style>
