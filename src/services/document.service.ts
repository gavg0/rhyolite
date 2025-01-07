import type { Tab } from "../types/tab";
import tabsStore from "../store/tabs.store";
import { ApiProvider } from "./api.service";
import TabService from "./tab.service";
import type { Document } from "../types/document";

const apiProvider = new ApiProvider();

const getAllDocumentTabs = async (): Promise<Tab[]> => {
    const tabs: Tab[] =  await apiProvider.getAllDocumentTabs();
    return tabsStore.updateTabsState(tabs);
}

const addNewDocumentTab = async (): Promise<void> => {
    try {
        const newTab: Tab = await apiProvider.addNewDocumentTab();

        await getAllDocumentTabs();
        tabsStore.updateCurrentTabState(newTab);

        await apiProvider.sendCurrentOpenTab(newTab.id);
    } catch (error) {
        console.error("Failed to create new document:", error);
    }
}

const deleteDocumentTab = async (): Promise<void> => {
    try {
        const currentTab: Tab | null = tabsStore.getCurrentTabState();
        if (currentTab === null) return;

        await apiProvider.deleteDocument(currentTab.id);
        const tabs = await getAllDocumentTabs();

        if (tabs.length > 0) {
            const lastTab = tabs[tabs.length - 1];
            tabsStore.updateCurrentTabState(lastTab);
        } else {
            await apiProvider.resetTabsOrderCount();
            await addNewDocumentTab();
        }
    } catch (error) {
        console.error("Failed to delete document:", error);
    }
}

const loadRecentDocuments = async (): Promise<void> => {
    try {
        const docs: Document[] = await apiProvider.loadRecentDocuments();

        if (docs.length > 0) {
            await apiProvider.resetTabsOrderCount();

            // Load each document as a tab
            for (const doc of docs) {
                await apiProvider.loadTab({ documentId: doc.id, documentTitle: doc.title });
            }

            // Update the tabs in UI
            await apiProvider.getAllDocumentTabs();

            // Load the last open document into the editor
            const open_tab: string = await apiProvider.getCurrentOpenTab();
            await TabService.switchTab(open_tab);
        } else {
            // If no documents exist, create a new tab
            await addNewDocumentTab();
        }
    } catch (error) {
        console.error("Failed to load documents:", error);
    }
}

const saveDocument = async (
    { documentId, documentTitle, documentContent }:
        { documentId: string; documentTitle: string; documentContent: any }
): Promise<void> => {
    await apiProvider.saveDocument({
        documentId,
        documentTitle,
        documentContent: typeof documentContent === "string" ? documentContent : JSON.stringify(documentContent),
    });
}

const loadDocument = async (documentId: string): Promise<Document | null> => {
    try {
        const doc =  await apiProvider.getDocumentContent(documentId);
        if(!doc) return null;

        if (isValidJSON(doc.content)) doc.content = JSON.parse(doc.content);
        return doc;
    } catch (error) {
        console.error("Failed to load document:", error);
        return null;
    }
}

const isValidJSON = (str: any): boolean => {
    if (typeof str !== 'string' || str.trim() === '') {
        return false; // Not a valid string
    }

    try {
        JSON.parse(str);
        return true; // The string is valid JSON
    } catch (e) {
        return false; // The string is not valid JSON
    }
}

export default {
    getAllDocumentTabs,
    addNewDocumentTab,
    deleteDocumentTab,
    loadRecentDocuments,
    saveDocument,
    loadDocument,
}