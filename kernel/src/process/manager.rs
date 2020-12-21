// SPDX-License-Identifier: GPL-3.0-or-later

use super::Process;
use alloc::collections::VecDeque;

struct Manager {
    tasks: VecDeque<Process>,
}
