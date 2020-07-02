extern crate embedded_hal as hal;
use hal::{timer};
use wasmi::{MemoryRef};
use nb;
use core::marker::PhantomData;

pub struct TimerSystem<Base>
    where
        Base:timer::CountDown<Time=u32>+timer::Cancel+timer::Periodic
{
    base:Base
}

impl<Base> TimerSystem<Base>
    where
        Base:timer::CountDown<Time=u32>+timer::Cancel+timer::Periodic
{

    fn start(&mut self, time:u32) -> Result<(),<Base as timer::CountDown>::Error> {
        self.base.try_start(time);
        Ok(())
    }

    fn wait(&mut self) -> Result<(),<Base as timer::CountDown>::Error> {
        self.base.try_wait();
        Ok(())
    }

    fn cancel(&mut self) -> Result<(),<Base as timer::CountDown>::Error> {
        self.base.try_cancel();
        Ok(())
    }
}

