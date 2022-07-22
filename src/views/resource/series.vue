<template>
    <NDataTable class="no-head" :columns="columns" :data="data" :bordered="false">
    </NDataTable>
</template>

<script lang="ts" setup>
import { NDataTable, DataTableColumn, NSpace, NButton, NTooltip } from 'naive-ui';
import { h, VNode, ref } from 'vue';
import { writeText } from '@tauri-apps/api/clipboard';
import { Series, SeriesFile } from '@/compositions/use-resource';

const tooltipContent = ref('点击复制');

const columns: DataTableColumn<Series>[] = [
    {
        key: 'name'
    },
    {
        key: 'size',
        width: 120
    },
    {
        key: 'files',
        render(row) {
            if (row.files.length < 0) {
                return '';
            }
            const buttons: VNode[] = [];
            row.files.forEach(file => {
                buttons.push(h(NTooltip, {
                    onUpdateShow: (value) => {
                        if (!value) {
                            tooltipContent.value = '点击复制';
                        }
                    }
                }, {
                    default: () => tooltipContent.value,
                    trigger: () => h(
                        NButton,
                        {
                            type: 'info',
                            size: 'small',
                            onClick: () => copy(file)
                        },
                        { default: () => file.way }
                    )
                }));
            });
            return h(NSpace, { style: { minWidth: '160px' } }, { default: () => buttons });
        }
    }
];

defineProps<{
    data: Series[]
}>();

const copy = (file: SeriesFile) => {
    writeText([file.address, file.password].filter(s => s !== '').join('\n'));
    tooltipContent.value = '已复制！';
};
</script>

<style lang="scss" scoped>
</style>
