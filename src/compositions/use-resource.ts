import { ref } from 'vue';
import useRequest from './use-request';

export interface SeriesFile {
    id: number,
    series_id: number,
    address: string,
    password: string,
    way: string,
}

export interface Series {
    id: number,
    format_id: number,
    episode: number,
    name: string,
    size: string,
    files: SeriesFile[],
}

export interface Format {
    id: number,
    season_id: number,
    format: string,
    series: Series[],
}

export interface Season {
    id: number,
    season: number,
    name: string,
    formats: Format[],
}

export interface Resource {
    id: number,
    name: string,
    original_name: string,
    alias_name: string,
    directors: string,
    writers: string,
    actors: string,
    types: string,
    released_at: string,
    summary: string,
    rating: number,
    seasons: Season[],
    channel: string,
    area: string,
}

export default (id: number) => {
    const resource = ref<Resource | null>(null);
    const { loading, request } = useRequest();
    request<Resource>('resource', { id })
        .then(response => {
            resource.value = response;
        })
        .catch(() => { });

    return { loading, resource };
}
