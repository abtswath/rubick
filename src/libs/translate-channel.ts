const channels: { [key: string]: string } = {
    tv: '电视剧',
    movie: '电影',
    openclass: '公开课',
    unknown: '未知'
};

export default (channel: string): string => {
    return channels[channel] || channels.unknown;
}
