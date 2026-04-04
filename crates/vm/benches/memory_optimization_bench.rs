//! Memory optimization benchmarks
//!
//! This benchmark validates the effectiveness of our memory optimizations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use neo_vm::evaluation_stack::EvaluationStack;
use neo_vm::memory_pool::{with_pools, VmMemoryPools};
use neo_vm::reference_counter::ReferenceCounter;
use neo_vm::stack_item::StackItem;

fn benchmark_memory_pool_allocation(c: &mut Criterion) {
    c.bench_function("memory_pool_byte_buffer_allocation", |b| {
        b.iter(|| {
            with_pools(|pools| {
                let mut buffer = pools.get_byte_buffer();
                buffer.extend_from_slice(b"test data for memory pool benchmark");
                buffer.len()
            })
        })
    });

    c.bench_function("memory_pool_instruction_buffer_allocation", |b| {
        b.iter(|| {
            with_pools(|pools| {
                let mut buffer = pools.get_instruction_buffer();
                // Simulate instruction loading
                for i in 0..10 {
                    buffer.push(neo_vm::instruction::Instruction::new(
                        neo_vm::op_code::OpCode::PUSH1,
                        i,
                        Some(vec![i as u8]),
                    ));
                }
                buffer.len()
            })
        })
    });

    c.bench_function("memory_pool_stack_item_allocation", |b| {
        b.iter(|| {
            with_pools(|pools| {
                let mut stack_items = pools.get_stack_item_vec();
                // Simulate stack operations
                for i in 0..5 {
                    stack_items.push(StackItem::from_int(i));
                }
                stack_items.len()
            })
        })
    });
}

fn benchmark_evaluation_stack(c: &mut Criterion) {
    let rc = ReferenceCounter::new();

    c.bench_function("evaluation_stack_operations", |b| {
        b.iter(|| {
            let mut stack = EvaluationStack::new(rc.clone());

            // Simulate typical stack operations
            for i in 0..black_box(20) {
                stack.push(StackItem::from_int(i));
            }

            // Pop some items
            for _ in 0..black_box(10) {
                if let Ok(_) = stack.pop() {
                    // Item popped successfully
                }
            }

            stack.size()
        })
    });
}

fn benchmark_memory_pool_performance(c: &mut Criterion) {
    c.bench_function("memory_pool_hit_ratio_measurement", |b| {
        b.iter(|| {
            with_pools(|pools| {
                // Perform some allocations to test hit ratio
                for _ in 0..50 {
                    let _buffer = pools.get_byte_buffer();
                }

                let metrics = pools.performance_metrics();
                black_box(metrics.overall_efficiency)
            })
        })
    });
}

criterion_group!(
    memory_benches,
    benchmark_memory_pool_allocation,
    benchmark_evaluation_stack,
    benchmark_memory_pool_performance
);
criterion_main!(memory_benches);
