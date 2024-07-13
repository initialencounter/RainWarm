type FileTileMap = FileTile[];


interface FileTile {
    name: string;
    lastModified: string;
    md5: string;
    color: string;
}


export type {FileTileMap, FileTile}