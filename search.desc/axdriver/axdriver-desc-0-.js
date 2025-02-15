searchState.loadedDescShard("axdriver", 0, "ArceOS device drivers.\nA structure that contains all device drivers, organized by …\nThe unified type of the block storage devices.\nA structure that contains all device drivers of a certain …\nA unified enum that represents different categories of …\nThe unified type of the graphics display devices.\nThe unified type of the NIC devices.\nBlock storage device.\nGraphic display device.\nNetwork card device.\nAll block device drivers.\nReturns the device model used, either <code>dyn</code> or <code>static</code>.\nAll graphics device drivers.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs a block device.\nConstructs a display device.\nConstructs a network device.\nConstructs the container from one device.\nProbes and initializes all device drivers, returns the …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns whether the container is empty.\nReturns number of devices in this container.\nAll network device drivers.\nDevice driver prelude that includes some traits and types.\nTakes one device out of the container (will remove it from …\nTry again, for non-blocking APIs.\nAn entity already exists.\nThe unified type of the block storage devices.\nThe unified type of the graphics display devices.\nThe unified type of the NIC devices.\nBad internal state.\nCommon operations that require all device drivers to …\nBlock storage device (e.g., disk).\nOperations that require a block storage device driver to …\nCharacter device (e.g., serial port).\nThe error type for device operation failures.\nA specialized <code>Result</code> type for device operations.\nAll supported device types.\nGraphic display device (e.g., GPU)\nOperations that require a graphics device driver to …\nContains the error value\nInvalid parameter/argument.\nInput/output error.\nNetwork device (e.g., ethernet card).\nOperations that require a network device (NIC) driver to …\nNot enough space/cannot allocate memory (DMA).\nContains the success value\nDevice or resource is busy.\nThis operation is unsupported or unimplemented.\nAllocate a memory buffer of a specified size for network …\nThe size of each block in bytes.\nWhether can receive packets.\nWhether can transmit packets.\nThe name of the device.\nThe type of the device.\nGet the framebuffer.\nFlush framebuffer to the screen.\nFlushes the device to write all pending data to the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the display information.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe ethernet address of the NIC.\nWhether need to flush the framebuffer to the screen.\nThe number of blocks in this storage device.\nReads blocked data from the given block.\nReceives a packet from the network and store it in the …\nGives back the <code>rx_buf</code> to the receive queue for later …\nPoll the transmit queue and gives back the buffers for …\nSize of the receive queue.\nTransmits a packet in the buffer to the network, without …\nSize of the transmit queue.\nWrites blocked data to the given block.")