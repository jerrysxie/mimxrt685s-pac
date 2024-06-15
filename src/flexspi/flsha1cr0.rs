#[doc = "Register `FLSHA1CR0` reader"]
pub type R = crate::R<Flsha1cr0Spec>;
#[doc = "Register `FLSHA1CR0` writer"]
pub type W = crate::W<Flsha1cr0Spec>;
#[doc = "Field `FLSHSZ` reader - Flash Size in KByte."]
pub type FlshszR = crate::FieldReader<u32>;
#[doc = "Field `FLSHSZ` writer - Flash Size in KByte."]
pub type FlshszW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - Flash Size in KByte."]
    #[inline(always)]
    pub fn flshsz(&self) -> FlshszR {
        FlshszR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - Flash Size in KByte."]
    #[inline(always)]
    #[must_use]
    pub fn flshsz(&mut self) -> FlshszW<Flsha1cr0Spec> {
        FlshszW::new(self, 0)
    }
}
#[doc = "Flash Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flsha1cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flsha1cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flsha1cr0Spec;
impl crate::RegisterSpec for Flsha1cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flsha1cr0::R`](R) reader structure"]
impl crate::Readable for Flsha1cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`flsha1cr0::W`](W) writer structure"]
impl crate::Writable for Flsha1cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLSHA1CR0 to value 0x0001_0000"]
impl crate::Resettable for Flsha1cr0Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
