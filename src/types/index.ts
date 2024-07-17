type FileTileMap = FileTile[];


interface FileTile {
    name: string;
    lastModified: string;
    md5: string;
    path: string;
    color: string;
    focus: boolean;
}

interface Link {
    link: string;
}
export type {FileTileMap, FileTile, Link}