#include "benchmark/BipartiteShockHashContender.h"
#include "benchmark/BipartiteShockHashFlatContender.h"
#include "benchmark/CmphContender.h"
#include "benchmark/DensePartitionedPTHashContender.h"
#include "benchmark/FiPSContender.h"
#include "benchmark/PTHashContender.h"
#include "benchmark/PartitionedPTHashContender.h"
#include "benchmark/RustFmphContender.h"
#include "benchmark/RustFmphGoContender.h"
#include "benchmark/RustPtrHashSimpleContender.h"
#include "benchmark/SIMDRecSplitContender.h"
#include "benchmark/SicHashContender.h"

#include <tlx/cmdline_parser.hpp>

int main(int argc, char** argv) {
	size_t N = 1e8;
	tlx::CmdlineParser cmd;
	cmd.add_bytes('n', "numKeys", N, "Number of objects");
	cmd.add_bytes('q', "numQueries", Contender::numQueries, "Number of queries to perform");
	cmd.add_bytes('t', "numThreads", Contender::numThreads,
	              "Number of threads to use for construction");
	cmd.add_flag('T', "skipTests", Contender::skipTests, "Skip testing PHF for validity");

	if(!cmd.process(argc, argv)) {
		return 1;
	}

	{
		FiPSContender<>(N, 1.5).run();
	}
	{
		FiPSContender<>(N, 2.0).run();
	}

	// Fast
	{
		RustPtrHashContender(N, false).run();
	}
	{
		RustPtrHashContenderStream(N, false).run();
	}
	// Compact
	{
		RustPtrHashContender(N, true).run();
	}
	{
		RustPtrHashContenderStream(N, true).run();
	}

	{
		PTHashContender<true, pthash::compact_compact>(N, 0.99, 4.0).run();
	}
	{
		PTHashContender<true, pthash::elias_fano>(N, 0.99, 5.0).run();
	}
	// Excluded for sure.
	// // {PTHashContender<true, pthash::elias_fano>(N, 0.99, 10.5).run();}

	{
		PartitionedPTHashContender<true, pthash::compact_compact>(N, 0.99, 4.0).run();
	}
	{
		PartitionedPTHashContender<true, pthash::elias_fano>(N, 0.99, 5.0).run();
	}

	{
		DensePartitionedPTHashContender<pthash::dense_interleaved<pthash::compact>,
		                                pthash::table_bucketer<pthash::opt_bucketer>>(N, 1.0, 3.9)
		    .run();
	}
	{
		DensePartitionedPTHashContender<pthash::dense_interleaved<pthash::rice>,
		                                pthash::table_bucketer<pthash::opt_bucketer>>(N, 1.0, 4.5)
		    .run();
	}
	{
		DensePartitionedPTHashContender<pthash::dense_interleaved<pthash::rice>,
		                                pthash::table_bucketer<pthash::opt_bucketer>>(N, 1.0, 6.5)
		    .run();
	}
	{
		DensePartitionedPTHashContender<pthash::dense_interleaved<pthash::compact>,
		                                pthash::table_bucketer<pthash::opt_bucketer>>(N, 1.0, 6.5)
		    .run();
	}
	{
		DensePartitionedPTHashContender<pthash::dense_interleaved<pthash::rice>,
		                                pthash::table_bucketer<pthash::opt_bucketer>>(N, 1.0, 7.0)
		    .run();
	}
	// {DensePartitionedPTHashContender<pthash::dense_interleaved<pthash::rice>,
	// pthash::table_bucketer<pthash::opt_bucketer>>(N, 1.0, 9.0).run();}

	{
		SicHashContender<true, 64>(N, 0.97, sichash::SicHashConfig().percentages(0.45, 0.31)).run();
	}
	{
		SicHashContender<true, 64>(N, 0.9, sichash::SicHashConfig().percentages(0.21, 0.78)).run();
	}

	{
		SIMDRecSplitContender<5>(N, 5).run();
	}
	{
		SIMDRecSplitContender<8>(N, 100).run();
	}
	// {SIMDRecSplitContender<14>(N, 2000).run();}

	// {BipartiteShockHashContender<42>(N, 2000).run();}
	// {BipartiteShockHashContender<64>(N, 2000).run();}

	{
		RustFmphContender(N, 1.0).run();
	}
	{
		RustFmphContender(N, 2.0).run();
	}

	{
		RustFmphGoContender(N, 1.0).run();
	}
	{
		RustFmphGoContender(N, 2.0).run();
	}

	{
		CmphContender(N, 1.0, "CHD", CMPH_CHD, 1.0, 3, true).run();
	}
	// {CmphContender(N, 1.0, "CHD", CMPH_CHD, 1.0, 5, true).run();}

	{
		BipartiteShockHashFlatContender<64>(N).run();
	}
	// {BipartiteShockHashFlatContender<100>(N).run();}

	return 0;
}
