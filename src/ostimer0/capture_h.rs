#[doc = "Register `CAPTURE_H` reader"]
pub type R = crate::R<CaptureHSpec>;
#[doc = "Field `CAPTURE_VALUE` reader - A read reflects the value of the upper 32 bits of the central EVTIMER at the time the last capture signal was generated by the CPU. A separate pair of CAPTURE registers are implemented for each CPU. Each CPU reads its own capture value at the same pair of addresses."]
pub type CaptureValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - A read reflects the value of the upper 32 bits of the central EVTIMER at the time the last capture signal was generated by the CPU. A separate pair of CAPTURE registers are implemented for each CPU. Each CPU reads its own capture value at the same pair of addresses."]
    #[inline(always)]
    pub fn capture_value(&self) -> CaptureValueR {
        CaptureValueR::new(self.bits)
    }
}
#[doc = "Local Capture High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`capture_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaptureHSpec;
impl crate::RegisterSpec for CaptureHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capture_h::R`](R) reader structure"]
impl crate::Readable for CaptureHSpec {}
#[doc = "`reset()` method sets CAPTURE_H to value 0"]
impl crate::Resettable for CaptureHSpec {
    const RESET_VALUE: u32 = 0;
}
