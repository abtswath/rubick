<template>
    <NPageHeader @back="router.go(-1)">
        <template #title>
            {{ resource?.name }}
        </template>
        <template #subtitle>
            {{ resource?.original_name }}
        </template>
        <div class="info-header">
            <div class="info-header-item">
                <NIcon size="18">
                    <MovieCreationOutlined />
                </NIcon>
                <span>{{ resource?.channel && translateChannel(resource.channel) }}</span>
            </div>
            <div class="info-header-item">
                <NIcon size="18">
                    <AccessTimeOutlined />
                </NIcon>
                <span>{{ resource?.released_at }}</span>
            </div>
            <div class="info-header-item">
                <NIcon size="18">
                    <CategoryOutlined />
                </NIcon>
                <span>{{ resource?.types }}</span>
            </div>
        </div>
        <div class="info">
            <NImage width="160" :src="resource?.pic" />
            <div class="info-text">
                <p class="rating">
                    <span>豆瓣评分：{{ resource?.rating }}</span>
                    <NRate readonly :value="(resource?.rating || 0) / 2" allow-half />
                </p>
                <p>
                    <NEllipsis :tooltip="{ contentStyle: { maxWidth: '300px' } }" :line-clamp="1">导演：{{ resource?.directors }}
                    </NEllipsis>
                </p>
                <p>
                    <NEllipsis :tooltip="{ contentStyle: { maxWidth: '300px' } }" :line-clamp="1">编剧：{{ resource?.writers }}
                    </NEllipsis>
                </p>
                <p>
                    <NEllipsis :tooltip="{ contentStyle: { maxWidth: '300px' } }" :line-clamp="1">演员：{{ resource?.actors }}
                    </NEllipsis>
                </p>
                <p class="summary">
                    简介：
                    {{ resource?.summary }}
                </p>
            </div>
        </div>
    </NPageHeader>
</template>

<script lang="ts" setup>
import { watchEffect } from 'vue';
import { NPageHeader, NImage, NIcon, NEllipsis, NRate } from 'naive-ui';
import { MovieCreationOutlined, AccessTimeOutlined, CategoryOutlined } from '@vicons/material';
import { useRouter } from 'vue-router';
import translateChannel from '@/libs/translate-channel';
import useResource from '@/compositions/use-resource';

const router = useRouter();

const props = defineProps<{
    id: number
}>();

console.log(props);
const { loading, resource } = useResource(props.id);
watchEffect(() => {
    console.log(resource.value)
});
</script>

<style lang="scss" scoped>
.info-header {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    margin-bottom: 16px;

    .info-header-item {
        width: auto;
        margin-right: 36px;
        line-height: 18px;
        display: flex;

        >span {
            margin-left: 8px;
        }
    }
}

.info {
    display: flex;
    align-items: flex-start;
    justify-content: flex-start;

    &-text {
        width: calc(100% - 160px);
        padding-left: 20px;

        p {
            margin: 0;
            display: flex;
        }

        p.rating {

            span {
                margin-right: 8px;
            }
        }

        p.summary {
            white-space: pre-line;
        }
    }
}
</style>
