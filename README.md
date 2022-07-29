# evtx-parser

- converting evtx files to csv.

## Usage
```
USAGE:
    evtx-parser --dir <DIR>

OPTIONS:
    -d, --dir <DIR>    target evtx dir path
    -h, --help         Print help information
```
## How to use([from release](https://github.com/fukusuket/evtx-parser/releases))

1. Download zip [from release page](https://github.com/fukusuket/evtx-parser/releases), and unzip.
2. ./evtx-parser -d ./sample

then output evtx converted csv files to ./target directory.

## How to use(from source)

1. 
2. git clone https://github.com/fukusuket/evtx-parser.git
3. cd evtx-parser
4. cargo run -- -d ./samples

- You must have c++ build tool installed as below(on windows).
  - https://docs.microsoft.com/ja-jp/windows/dev-environment/rust/setup

## Input

- evtx files directory.

## Output
- example output Security.evtx
```
"event_record_id","timestamp","data"
"1","2022-06-08 23:46:15.972823 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""CommandLine"":"""",""MandatoryLabel"":""S-1-16-16384"",""NewProcessId"":""0x7c"",""NewProcessName"":""Registry"",""ParentProcessName"":"""",""ProcessId"":""0x4"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x0"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0"",""TokenElevationType"":""%%1936""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4688,""EventRecordID"":1,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":132}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:15.972823Z""}},""Version"":2}}}"
"2","2022-06-08 23:46:15.972843 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""ProcessId"":""0x4"",""ProcessName"":"""",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x3e7"",""TargetProcessId"":""0x7c"",""TargetProcessName"":""Registry"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4696,""EventRecordID"":2,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":132}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:15.972843Z""}},""Version"":0}}}"
"3","2022-06-08 23:46:15.972846 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""AdvancedOptions"":""%%1843"",""ConfigAccessPolicy"":""%%1846"",""DisableIntegrityChecks"":""%%1843"",""FlightSigning"":""%%1843"",""HypervisorDebug"":""%%1843"",""HypervisorLaunchType"":""%%1848"",""HypervisorLoadOptions"":""-"",""KernelDebug"":""%%1843"",""LoadOptions"":""-"",""RemoteEventLogging"":""%%1843"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TestSigning"":""%%1843"",""VsmLaunchType"":""%%1848""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4826,""EventRecordID"":3,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":132}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13573,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:15.972846Z""}},""Version"":0}}}"
"4","2022-06-08 23:46:15.975129 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""CommandLine"":"""",""MandatoryLabel"":""S-1-16-16384"",""NewProcessId"":""0x184"",""NewProcessName"":""C:\\Windows\\System32\\smss.exe"",""ParentProcessName"":"""",""ProcessId"":""0x4"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x0"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0"",""TokenElevationType"":""%%1936""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4688,""EventRecordID"":4,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":132}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:15.975129Z""}},""Version"":2}}}"
"5","2022-06-08 23:46:16.454369 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""CommandLine"":"""",""MandatoryLabel"":""S-1-16-16384"",""NewProcessId"":""0x1a0"",""NewProcessName"":""C:\\Windows\\System32\\autochk.exe"",""ParentProcessName"":""C:\\Windows\\System32\\smss.exe"",""ProcessId"":""0x184"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x0"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0"",""TokenElevationType"":""%%1936""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4688,""EventRecordID"":5,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":268}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:16.454369Z""}},""Version"":2}}}"
"6","2022-06-08 23:46:16.700120 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""CommandLine"":"""",""MandatoryLabel"":""S-1-16-16384"",""NewProcessId"":""0x1e0"",""NewProcessName"":""C:\\Windows\\System32\\setupcl.exe"",""ParentProcessName"":""C:\\Windows\\System32\\smss.exe"",""ProcessId"":""0x184"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x0"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0"",""TokenElevationType"":""%%1936""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4688,""EventRecordID"":6,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":404}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:16.700120Z""}},""Version"":2}}}"
"7","2022-06-08 23:46:16.720257 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""CommandLine"":"""",""MandatoryLabel"":""S-1-16-16384"",""NewProcessId"":""0x1ec"",""NewProcessName"":""C:\\Windows\\System32\\smss.exe"",""ParentProcessName"":""C:\\Windows\\System32\\smss.exe"",""ProcessId"":""0x184"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x0"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0"",""TokenElevationType"":""%%1936""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4688,""EventRecordID"":7,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":360}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:16.720257Z""}},""Version"":2}}}"
"8","2022-06-08 23:46:16.751452 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""CommandLine"":"""",""MandatoryLabel"":""S-1-16-16384"",""NewProcessId"":""0x1fc"",""NewProcessName"":""C:\\Windows\\System32\\csrss.exe"",""ParentProcessName"":""C:\\Windows\\System32\\smss.exe"",""ProcessId"":""0x1ec"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x0"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0"",""TokenElevationType"":""%%1936""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4688,""EventRecordID"":8,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":256}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:16.751452Z""}},""Version"":2}}}"
"9","2022-06-08 23:46:16.768206 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""CommandLine"":"""",""MandatoryLabel"":""S-1-16-16384"",""NewProcessId"":""0x238"",""NewProcessName"":""C:\\Windows\\System32\\smss.exe"",""ParentProcessName"":""C:\\Windows\\System32\\smss.exe"",""ProcessId"":""0x184"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x0"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0"",""TokenElevationType"":""%%1936""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4688,""EventRecordID"":9,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":256}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:16.768206Z""}},""Version"":2}}}"
"10","2022-06-08 23:46:16.769473 UTC","{""Event"":{""#attributes"":{""xmlns"":""http://schemas.microsoft.com/win/2004/08/events/event""},""EventData"":{""CommandLine"":"""",""MandatoryLabel"":""S-1-16-16384"",""NewProcessId"":""0x244"",""NewProcessName"":""C:\\Windows\\System32\\wininit.exe"",""ParentProcessName"":""C:\\Windows\\System32\\smss.exe"",""ProcessId"":""0x1ec"",""SubjectDomainName"":""-"",""SubjectLogonId"":""0x3e7"",""SubjectUserName"":""-"",""SubjectUserSid"":""S-1-5-18"",""TargetDomainName"":""-"",""TargetLogonId"":""0x0"",""TargetUserName"":""-"",""TargetUserSid"":""S-1-0-0"",""TokenElevationType"":""%%1936""},""System"":{""Channel"":""Security"",""Computer"":""FUKUSUK-K0TUHAN"",""Correlation"":null,""EventID"":4688,""EventRecordID"":10,""Execution"":{""#attributes"":{""ProcessID"":4,""ThreadID"":32}},""Keywords"":""0x8020000000000000"",""Level"":0,""Opcode"":0,""Provider"":{""#attributes"":{""Guid"":""54849625-5478-4994-A5BA-3E3B0328C30D"",""Name"":""Microsoft-Windows-Security-Auditing""}},""Security"":null,""Task"":13312,""TimeCreated"":{""#attributes"":{""SystemTime"":""2022-06-08T23:46:16.769473Z""}},""Version"":2}}}"
```