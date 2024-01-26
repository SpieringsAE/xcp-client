use c_enum::c_enum;

c_enum! {
    #[derive(Copy,Clone,PartialEq, Eq)]
    pub enum XCPCommand: u8 {
        Connect = 0xff,
        Disconnect = 0xfe,
        GetStatus = 0xfd,
        Synch = 0xfc,
        ///Optional
        GetCommModeInfo = 0xfb,
        ///Optional
        GetID = 0xfa,
        ///Optional
        SetRequest = 0xf9,
        ///Optional
        GetSeed = 0xf8,
        ///Optional
        Unlock = 0xf7,
        ///Optional
        SetMTA = 0xf6,
        ///Optional
        Upload = 0xf5,
        ///Optional
        ShortUpload = 0xf4,
        ///Optional
        BuildChecksum = 0xf3,
        ///Optional
        TransportLayerCMD = 0xf2,
        ///Optional
        UserCMD = 0xf1,
        ///Optional
        GetVersion = 0xc0,
        //Optional
        GetVersionExtra = 0x00,

        Download = 0xf0,
        ///Optional
        DownloadNext = 0xef,
        ///Optional
        DownloadMax = 0xee,
        ///Optional
        ShortDownload = 0xed,
        ///Optional
        ModifyBits,

        SetCalPage = 0xeb,
        GetCalPage = 0xea,
        ///Optional
        GetPagProcessorInfo = 0xe9,
        ///Optional
        GetSegmentInfo = 0xe8,
        ///Optional
        GetPageInfo = 0xe7,
        ///Optional
        SetSegmentMode = 0xe6,
        ///Optional
        GetSegmentMode = 0xe5,
        ///Optional
        CopyCalPage = 0xe4,

        SetDAQPtr = 0xe2,
        WriteDAQ = 0xe1,
        SetDAQListMode = 0xe0,
        StartStopDAQList = 0xde,
        StartStopSynch = 0xdd,
        ///Optional
        WriteDAQMultiple = 0xc7,
        ///Optional
        ReadDaq = 0xdb,
        ///Optional
        GetDAQClock = 0xdc,
        ///Optional
        GetDAQProcessorInfo = 0xda,
        ///Optional
        GetDAQResolutionInfo = 0xd7,
        ///Optional
        GetDAQListMode = 0xdf,
        ///Optional
        GetDAQEventInfo = 0xd7,
        ///Optional
        DTOCTRProperties = 0xc5,
        ///Optional
        SetDAQPackedMode = 0xc0,
        ///Optional
        SetDAQPackedModeExtra = 0x01,
        ///Optional
        GetDAQPackedMode = 0xc0,
        ///Optional
        GetDAQPackedModeExtra = 0x02,

        ClearDAQList = 0xe3,
        ///Optional
        GetDAQListInfo = 0xd8,

        ///Optional
        FreeDAQ = 0xd6,
        ///Optional
        AllocDAQ = 0xd5,
        ///Optional
        AllocODT = 0xd4,
        ///Optional
        AllocODTEntry = 0xd3,

        ProgramStart = 0xd2,
        ProgramClear = 0xd1,
        Program = 0xd0,
        ProgramReset = 0xcf,
        ///Optional
        GetPGMProcessorInfo = 0xce,
        ///Optional
        GetSectorInfo = 0xcd,
        ///Optional
        ProgramPrepare = 0xcc,
        ///Optional
        ProgramFormat = 0xcb,
        ///Optional
        ProgramNext = 0xca,
        ///Optional
        ProgramMax = 0xc9,
        ///Optional
        ProgramVerify = 0xc8,

        ///Optional
        TimeCorrelationProperties = 0xc6,

        ASAMAEMCD1XCPAsSWDBGOverXCP = 0xc0,
        ASAMAEMCD1XCPAsSWDBGOverXCPExtra = 0xfc,
        ASAMAEMCD1PODBS = 0xc0,
        ASAMAEMCD1PODBSExtra = 0xfd,
    }
}

c_enum! {
    #[derive(Copy,Clone,PartialEq, Eq)]
    pub enum XCPResponse: u8 {
        Result = 0xff,
        Error = 0xfe,
        Event = 0xfd,
        Service = 0xfc,
    }
}
