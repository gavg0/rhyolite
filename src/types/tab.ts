export interface Tab {
    id: string;
    title: string;
    order?: number;
    tabType?: TabType;
    documentId?: string;
}

export enum TabType {
    Document = "Document",
    Other = "Other",
}