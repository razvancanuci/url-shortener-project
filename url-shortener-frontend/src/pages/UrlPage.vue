<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { api } from 'boot/axios'
import { Notify, useQuasar } from 'quasar'
import type { AxiosResponse } from 'axios'
import { useI18n } from 'vue-i18n'

const longUrl = ref('');
const shortUrl = ref('');
const qrCode = ref('');

const $q = useQuasar();

const { t, locale } = useI18n({ useScope: 'global' })

const shortenUrl = async () => {
  const request = {url: longUrl.value};
  const response: AxiosResponse<any, any> = await api.post('/api/url', request).catch(err => {
    console.log(err);
    Notify.create({
      message: err.response.data.message,
      color: 'negative',
      position: 'top',
      closeBtn: 'X'
    });
    return {status: 400, data: null} as AxiosResponse<any, any>;
  });
  shortUrl.value = response.data.value.shortUrl as string;
  qrCode.value = response.data.value.qrCodeImage as string;

  Notify.create({
    message: t('successShorten'),
    color: 'positive',
    position: 'top',
    closeBtn: 'X'
  });
};

const copyToClipboard = async (text: string) => {
  await navigator.clipboard.writeText(text);
  Notify.create({
    message: t('clipboardCopy'),
    color: 'positive',
    position: 'top-right',
    closeBtn: 'X'
  });
};

const downloadImage = (qr: string) => {

  const a = document.createElement('a');
  a.href = qr;
  a.download = (qr.split('/').pop()) as string;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
};

onMounted(() => {
  locale.value = $q.lang.getLocale() as string;
});

</script>

<template>
  <q-layout>
  <q-page-container>

  <q-page class="responsive-page flex q-mx-xl q-my-xl">
    <q-card class="responsive-card q-pa-md shadow-4 q-mr-xl">
      <q-card-section>
        <div class="text-h4 text-center">{{t('formTitle')}}</div>
      </q-card-section>

      <q-card-section>
        <q-input v-model="longUrl" :label="t('inputLabel')" outlined dense :rules="[
          (val) => !!val || t('validationRequiredUrl'),
          (val) => /^https?:\/\/.+\..+/.test(val) || t('validationValidUrl')
        ]" />
        <q-btn class="full-width q-mt-md" color="primary" @click="shortenUrl">
          {{t('shortenButton')}}
        </q-btn>
      </q-card-section>

      <q-card-section :style="{visibility: shortUrl ? 'visible': 'hidden'}">
        <q-list bordered separator>
          <q-item class="flex justify-between">
            <q-item-section side>
              <q-btn icon="content_copy" dense flat @click="copyToClipboard(shortUrl)" />
            </q-item-section>
            <q-item-section>
              <span class="q-ml-md text-h6">{{shortUrl}}</span>
            </q-item-section>
              <q-img width="64px" height="64px" :src="qrCode" @click="downloadImage(qrCode)"/>
          </q-item>
        </q-list>
      </q-card-section>
      <q-card-section>
        <h6>{{t('formFirstParagraph')}}</h6>
        <h6>{{t('formSecondParagraph')}}</h6>
        <h6>{{t('formThirdParagraph')}}</h6>
      </q-card-section>
      <q-footer class="bg-white text-center text-black">
        Răzvan Cănuci
      </q-footer>
    </q-card>
    <div class="text-center items-center">
      <h2 class="text-white">{{t('mainTitle')}}</h2>
      <h4 class="text-white">{{t('mainFirstParagraph')}}</h4>
      <h4 class="text-white">{{t('mainSecondParagraph')}}</h4>
      <h5 class="text-white">{{t('mainThirdParagraph')}}</h5>
    </div>

  </q-page>
  </q-page-container>
  </q-layout>
</template>

<style scoped>
h1, h2, h3, h4, h5, h6 {
  font-family: 'FontAwesome','serif';
  font-style: italic;
}

.responsive-card {
  max-width: 31%;
  max-height: 50%;
  min-height: 40%;
}
@media (max-width: 1400px) {

  .responsive-card {
    max-width: 100%;
    margin: 0;
  }
}

@media (max-width: 600px) {
  .responsive-page {
    margin: 0;
  }
}

</style>
