import type { Tab } from "../types/tab";
import { invoke } from "@tauri-apps/api/core";
import type { Document } from "../types/document";
import type { IApiServiceProvider } from "./api.interface";

export class TauriInvokeServiceProvider implements IApiServiceProvider {
    async addNewDocumentTab(): Promise<Tab> {
        return await invoke<Tab>("new_tab");
    }

    async getAllDocumentTabs(): Promise<Tab[]> {
        return await invoke<Tab[]>("get_tabs");
    }

    async sendCurrentOpenTab(tabId: string) {
        await invoke("send_current_open_tab", { id: tabId });
    }

    async getDocumentContent(tabId: string): Promise<Document | null> {
        return await invoke<Document | null>(
            "get_document_content",
            { id: tabId },
        );
    }

    async saveDocument(
        { documentId, documentTitle, documentContent }: { documentId: string; documentTitle: string; documentContent: string }
    ): Promise<void> {
        await invoke("save_document", {
            id: documentId,
            title: documentTitle,
            content: documentContent,
        });
        await invoke("update_tab_title", {
            id: documentId,
            title: documentTitle,
        });
    }
    
    async loadRecentDocuments(): Promise<Document[]>{
        return await invoke<Document[]>("load_recent_files");
    }

    // Not required Anymore, order of tab handled by the back end.
    // async resetTabsOrderCount() {
    //     await invoke("reset_tab_order_count");
    // }

    async loadTab(
        { documentId, documentTitle }: { documentId: string; documentTitle: string }
    ): Promise<void> {
        await invoke("load_tab", {
            id: documentId,
            title: documentTitle,
        });
    }

    async deleteDocument(documentId: string){
        await invoke("delete_document", { id: documentId });
    }

    async getCurrentOpenTab(): Promise<string> {
        return await invoke("get_current_open_tab");
    }
}
