pub const Dummyval: &[u16] = &[
    5601, 9300, 80, 23, 443, 21, 22, 25, 3389];

// TODO:
// 1. figure out appropriate structures and libraries for parsing service probe format into rust
// 2. write parsing logic, include PCRE, to get file contents into rust.
    // NOTE: pay attention to RE compilation runtime performance.  see if lazy compile is possible.
// 3. wire parsed probes into main scanner logic
    // probably need to add results housing structure here, replace current println approach
// 4. Start with hardcoded file to read in, but consider how to automatically find folder of probe files
    // either pass in as CLI arg, check environment vars, or some other approach?
// 4. Right now the scanner only does TCP, so only TCP probes make sense. 
    // add parser support for all, but just short circuit non-tcp probes for now.