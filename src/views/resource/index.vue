<template>
    <NPageHeader @back="router.go(-1)">
        <template #title>
            <NSkeleton text :width="80" v-if="loading" />
            <template v-else>{{ resource?.name }}</template>
        </template>
        <template #subtitle>
            <NSkeleton text :width="140" v-if="loading" />
            <template v-else>{{ resource?.original_name }}</template>
        </template>
        <div class="info-header">
            <div class="info-header-item">
                <NIcon size="18">
                    <MovieCreationOutlined />
                </NIcon>
                <span>
                    <NSkeleton text :width="80" v-if="loading" />
                    <template v-else>{{ resource?.channel && translateChannel(resource.channel) }}</template>
                </span>
            </div>
            <div class="info-header-item">
                <NIcon size="18">
                    <AccessTimeOutlined />
                </NIcon>
                <span>
                    <NSkeleton text :width="80" v-if="loading" />
                    <template v-else>{{ resource?.released_at }}</template>
                </span>
            </div>
            <div class="info-header-item">
                <NIcon size="18">
                    <CategoryOutlined />
                </NIcon>
                <span>
                    <NSkeleton text :width="80" v-if="loading" />
                    <template v-else>{{ resource?.types }}</template>
                </span>
            </div>
        </div>
        <div class="info">
            <div>
                <NSkeleton text :width="160" :height="237" v-if="loading" />
                <NImage v-else :width="160" preview-disabled :src="resource?.pic" />
                <div class="info-action">
                    <NButton @click="unFavorite" v-if="resource?.favorite" type="primary" style="width: 100%;">
                        <template #icon>
                            <NIcon>
                                <FavoriteOutlined />
                            </NIcon>
                        </template>
                        取消收藏
                    </NButton>
                    <NButton @click="favorite" v-else type="primary" style="width: 100%;">
                        <template #icon>
                            <NIcon>
                                <FavoriteBorderOutlined />
                            </NIcon>
                        </template>
                        收藏
                    </NButton>
                </div>
            </div>
            <div class="info-items">
                <p class="rating">
                    <span>豆瓣评分：{{ resource?.rating }}</span>
                    <NRate readonly :value="(resource?.rating || 0) / 2" allow-half />
                </p>
                <NEllipsis :tooltip="{ contentStyle: { maxWidth: '300px' } }" :line-clamp="1">
                    <label class="label">导演：</label>
                    <NSkeleton text :width="80" v-if="loading" />
                    <template v-else>{{ resource?.directors }}</template>
                </NEllipsis>
                <NEllipsis :tooltip="{ contentStyle: { maxWidth: '300px' } }" :line-clamp="1">
                    <label class="label">编剧：</label>
                    <NSkeleton text :width="80" v-if="loading" />
                    <template v-else>{{ resource?.writers }}</template>
                </NEllipsis>
                <NEllipsis :tooltip="{ contentStyle: { maxWidth: '300px' } }" :line-clamp="1">
                    <label class="label">演员：</label>
                    <NSkeleton text :width="80" v-if="loading" />
                    <template v-else>{{ resource?.actors }}</template>
                </NEllipsis>
                <p class="summary">
                    <label class="label">简介：</label>
                    <template v-if="loading">
                        <NSkeleton text width="calc(100% - 47px)" />
                        <NSkeleton text width="100%" :repeat="5" />
                        <NSkeleton text width="60%" />
                    </template>
                    <template v-else>{{ resource?.summary }}</template>
                </p>
            </div>
        </div>
        <template v-if="!loading">
            <NDivider />
            <Season :data="resource?.seasons || []" />
        </template>
    </NPageHeader>
</template>

<script lang="ts" setup>
import { NButton, NPageHeader, NSkeleton, NImage, NIcon, NEllipsis, NRate, NDivider } from "naive-ui";
import { MovieCreationOutlined, AccessTimeOutlined, CategoryOutlined, FavoriteBorderOutlined, FavoriteOutlined } from "@vicons/material";
import { useRouter } from "vue-router";
import translateChannel from "@/libs/translate-channel";
import useResource from "@/compositions/use-resource";
import Season from "./season.vue";
import useFavorite from '@/compositions/use-favorite';

const router = useRouter();

const props = defineProps<{
    id: number;
}>();

const { loading, resource } = useResource(props.id);
const { favorite, unFavorite } = useFavorite(resource);
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

    &-action {
        width: 160px;
    }

    &-items {
        width: calc(100% - 160px);
        padding-left: 20px;
        display: flex;
        flex-direction: column;

        label.label {
            width: 50px;
        }

        p {
            margin: 0;
        }

        p.rating {
            display: flex;

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
