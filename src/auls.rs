#![allow(non_snake_case)]
pub struct CMemref {
    pub m_exedit: *mut u32,

    pub m_Exedit_StaticFilterTable: u32,
    pub m_Exedit_SortedObjectTable_LayerIndexEnd: u32,
    pub m_Exedit_AliasNameBuffer: u32,
    pub m_Exedit_SortedObjectCount: u32,
    pub m_Exedit_ObjDlg_CommandTarget: u32,
    pub m_Exedit_SortedObjectTable_LayerIndexBegin: u32,
    pub m_Exedit_ObjDlg_FilterStatus: u32,
    pub m_Exedit_SortedObjectTable: u32,
    pub m_Exedit_ObjDlg_ObjectIndex: u32,
    pub m_Exedit_SceneSetting: u32,
    pub m_Exedit_LoadedFilterTable: u32,
    pub m_Exedit_LayerSetting: u32,
    pub m_Exedit_SceneDisplaying: u32,
    pub m_Exedit_TextBuffer: u32,
    pub m_Exedit_TraScript_ProcessingTrackBarIndex: u32,
    pub m_Exedit_TraScript_ProcessingObjectIndex: u32,
    pub m_Exedit_ScriptProcessingFilter: u32,
    pub m_Exedit_LuaState: u32,
    pub m_Exedit_ObjectBufferInfo: u32,
    pub m_Exedit_CameraZBuffer: u32,
    pub m_Exedit_UndoInfo: u32,

    pub m_Exedit_ObjectBufferInfo_exdata_size: u32,
    pub m_Exedit_ObjectBufferInfo_max_data_num: u32,
    pub m_Exedit_ObjectBufferInfo_data: u32,
    pub m_Exedit_ObjectBufferInfo_exdata: u32,

    pub m_Exedit_UndoInfo_current_id: u32,
    pub m_Exedit_UndoInfo_write_offset: u32,
    pub m_Exedit_UndoInfo_object_num: u32,
    pub m_Exedit_UndoInfo_buffer_ptr: u32,
    pub m_Exedit_UndoInfo_buffer_size: u32,
}

impl Default for CMemref {
    fn default() -> Self {
        CMemref {
            m_exedit: 0 as *mut u32,

            m_Exedit_StaticFilterTable: 0,
            m_Exedit_SortedObjectTable_LayerIndexEnd: 0,
            m_Exedit_AliasNameBuffer: 0,
            m_Exedit_SortedObjectCount: 0,
            m_Exedit_ObjDlg_CommandTarget: 0,
            m_Exedit_SortedObjectTable_LayerIndexBegin: 0,
            m_Exedit_ObjDlg_FilterStatus: 0,
            m_Exedit_SortedObjectTable: 0,
            m_Exedit_ObjDlg_ObjectIndex: 0,
            m_Exedit_SceneSetting: 0,
            m_Exedit_LoadedFilterTable: 0,
            m_Exedit_LayerSetting: 0,
            m_Exedit_SceneDisplaying: 0,
            m_Exedit_TextBuffer: 0,
            m_Exedit_TraScript_ProcessingTrackBarIndex: 0,
            m_Exedit_TraScript_ProcessingObjectIndex: 0,
            m_Exedit_ScriptProcessingFilter: 0,
            m_Exedit_LuaState: 0,
            m_Exedit_ObjectBufferInfo: 0,
            m_Exedit_CameraZBuffer: 0,
            m_Exedit_UndoInfo: 0,

            m_Exedit_ObjectBufferInfo_exdata_size: 0,
            m_Exedit_ObjectBufferInfo_max_data_num: 0,
            m_Exedit_ObjectBufferInfo_data: 0,
            m_Exedit_ObjectBufferInfo_exdata: 0,

            m_Exedit_UndoInfo_current_id: 0,
            m_Exedit_UndoInfo_write_offset: 0,
            m_Exedit_UndoInfo_object_num: 0,
            m_Exedit_UndoInfo_buffer_ptr: 0,
            m_Exedit_UndoInfo_buffer_size: 0,
        }
    }
}
